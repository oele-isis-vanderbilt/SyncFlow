use super::{secret, tokens_manager, user};
use crate::project::{self, project_crud};
use crate::users::oauth::github::{fetch_github_user, verify_user_token, GithubUser};
use crate::users::tokens_manager::{TokenTypes, UserInfo};
use crate::users::user::UserError;
use domain::models::User;
use infrastructure::DbPool;
use shared::deployment_config::DeploymentConfig;
use shared::project_models::{ProjectSummary, ProjectsSummary};
use shared::user_models::{
    ApiKeyRequest, ApiKeyResponse, ApiKeyResponseWithoutSecret, ProjectInfo, ProjectRequest,
    RefreshTokenRequest, TokenResponse, UserProfile,
};
use shared::user_models::{LoginRequest, SignUpRequest};
use std::sync::Arc;
use uuid::Uuid;

pub struct AccountService {
    pool: Arc<DbPool>,
    config: DeploymentConfig,
    tokens_manager: tokens_manager::JWTTokensManager,
}

impl AccountService {
    pub fn new(pool: Arc<DbPool>, config: DeploymentConfig) -> Self {
        let encryption_key = &config.encryption_key.clone();
        let jwt_expiration = config.jwt_expiration;
        let jwt_refresh_expiration = config.jwt_refresh_expiration;
        AccountService {
            pool,
            config,
            tokens_manager: tokens_manager::JWTTokensManager::new(
                encryption_key,
                jwt_expiration,
                jwt_refresh_expiration,
            ),
        }
    }

    pub fn signup(&self, request: SignUpRequest) -> Result<(), UserError> {
        user::signup(
            &request,
            &mut self.pool.get().unwrap(),
            &self.config.encryption_key,
        )
    }

    /// Logs in a user
    pub fn login(&self, request: LoginRequest) -> Result<TokenResponse, UserError> {
        let session_info = user::login(
            request,
            &mut self.pool.get().unwrap(),
            &self.config.encryption_key,
        )?;
        let conn = &mut self.pool.get().unwrap();
        let tokens = self
            .tokens_manager
            .generate_login_token_pairs(&session_info, conn)?;
        Ok(TokenResponse::bearer(tokens.0, tokens.1))
    }

    pub fn refresh_token(&self, request: RefreshTokenRequest) -> Result<TokenResponse, UserError> {
        let user_info = self
            .tokens_manager
            .verify_token(&request.refresh_token, &mut self.pool.get().unwrap())?;
        let sid = user_info
            .login_session
            .ok_or(UserError::LoginSessionNotFound(format!(
                "No login session found for the refresh token, {:?}",
                &request.refresh_token
            )))?;
        let conn = &mut self.pool.get().unwrap();
        let login_session_info = user::get_login_session_info(user_info.user_id, &sid, conn)?;
        self.tokens_manager
            .generate_login_token_pairs(&login_session_info, conn)
            .map(|t| TokenResponse::bearer(t.0, t.1))
    }

    /// Logs out a user
    pub fn logout(&self, token: &str) -> Result<(), user::UserError> {
        let conn = &mut self.pool.get().unwrap();
        let decoded_token = self.tokens_manager.verify_token(token, conn)?;
        let session_id = decoded_token
            .login_session
            .ok_or(UserError::TokenError("No login session found".to_string()))?;
        let _ = user::delete_login_session(&session_id, conn);
        Ok(())
    }

    pub fn get_user(&self, user_id: i32) -> Result<UserProfile, UserError> {
        user::get_user(user_id, &mut self.pool.get().unwrap()).map(Into::into)
    }

    pub fn get_pool(&self) -> Arc<DbPool> {
        self.pool.clone()
    }

    pub fn decode_token(&self, token: String) -> Result<TokenTypes, UserError> {
        self.tokens_manager.decode_token_unsafe(&token)
    }

    pub fn verify_token(&self, token_data: &str) -> Result<UserInfo, UserError> {
        self.tokens_manager
            .verify_token(token_data, &mut self.pool.get().unwrap())
    }

    pub fn generate_api_keys(
        &self,
        user_id: i32,
        request: &ApiKeyRequest,
    ) -> Result<ApiKeyResponse, UserError> {
        let mut api_key: ApiKeyResponse = user::generate_non_login_api_key(
            user_id,
            &self.config.encryption_key,
            Some(request.comment.clone()),
            &mut self.pool.get().unwrap(),
        )
        .map(Into::into)?;

        api_key.secret = self.decrypt_secret(&api_key.secret)?;

        Ok(api_key)
    }

    pub fn list_api_keys(
        &self,
        user_id: i32,
    ) -> Result<Vec<ApiKeyResponseWithoutSecret>, UserError> {
        let mut conn = self.pool.get().unwrap();
        user::get_all_api_keys(user_id, &mut conn)
            .map(|keys| keys.into_iter().map(Into::into).collect())
    }

    pub fn delete_api_key(
        &self,
        user_id: i32,
        key_id: &str,
    ) -> Result<ApiKeyResponseWithoutSecret, UserError> {
        user::delete_api_key(user_id, key_id, &mut self.pool.get().unwrap()).map(Into::into)
    }

    pub fn create_project_api_key(
        &self,
        user_id: i32,
        project_id: &str,
        request: &ApiKeyRequest,
    ) -> Result<ApiKeyResponse, UserError> {
        use project_crud::Encryptable;
        let mut api_key = project_crud::create_api_key(
            user_id,
            project_id,
            Some(request.comment.clone()),
            &self.config.encryption_key,
            &mut self.pool.get().unwrap(),
        )?;

        api_key
            .decrypt(&self.config.encryption_key)
            .map_err(|e| UserError::SecretError(e.to_string()))?;

        Ok(api_key.into())
    }

    pub fn list_project_api_keys(
        &self,
        user_id: i32,
        project_id: &str,
    ) -> Result<Vec<ApiKeyResponseWithoutSecret>, UserError> {
        let keys =
            project_crud::list_all_api_keys(user_id, project_id, &mut self.pool.get().unwrap())?;
        Ok(keys.into_iter().map(Into::into).collect())
    }

    pub fn delete_project_api_key(
        &self,
        user_id: i32,
        project_id: &str,
        key_id: i32,
    ) -> Result<ApiKeyResponseWithoutSecret, UserError> {
        let key = project_crud::delete_api_key(
            user_id,
            project_id,
            key_id,
            &mut self.pool.get().unwrap(),
        )?;
        Ok(key.into())
    }

    pub fn create_project(
        &self,
        user_id: i32,
        new_project_request: &ProjectRequest,
    ) -> Result<ProjectInfo, UserError> {
        let project = project::project_crud::create_project(
            user_id,
            new_project_request,
            &self.config.encryption_key,
            &mut self.pool.get().unwrap(),
        )?;

        Ok(project.into())
    }

    pub fn get_projects(&self, user_id: i32) -> Result<Vec<ProjectInfo>, UserError> {
        let projects =
            project::project_crud::list_projects(user_id, &mut self.pool.get().unwrap())?;
        Ok(projects.into_iter().map(Into::into).collect())
    }

    pub fn get_project(&self, user_id: i32, project_id: &str) -> Result<ProjectInfo, UserError> {
        let project =
            project::project_crud::get_project(user_id, project_id, &mut self.pool.get().unwrap())?;
        Ok(project.into())
    }

    pub fn delete_project(&self, user_id: i32, project_id: &str) -> Result<ProjectInfo, UserError> {
        let project = project::project_crud::delete_project(
            user_id,
            project_id,
            &mut self.pool.get().unwrap(),
        )?;
        Ok(project.into())
    }

    pub fn summarize_projects(&self, user_id: i32) -> Result<ProjectsSummary, UserError> {
        let summary =
            project::project_crud::summarize_projects(user_id, &mut self.pool.get().unwrap())?;
        Ok(summary)
    }

    pub async fn summarize_project(&self, project_id: &str) -> Result<ProjectSummary, UserError> {
        let summary = project::project_crud::summarize_project(
            project_id,
            &mut self.pool.get().unwrap(),
            &self.config.encryption_key,
        )
        .await?;

        Ok(summary)
    }

    pub fn update_project(
        &self,
        user_id: i32,
        project_id: Uuid,
        new_project_request: &ProjectRequest,
    ) -> Result<ProjectInfo, UserError> {
        let project = project::project_crud::update_project(
            user_id,
            project_id,
            new_project_request,
            &self.config.encryption_key,
            &mut self.pool.get().unwrap(),
        )?;
        Ok(project.into())
    }

    pub fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, UserError> {
        user::create_user(username, email, password, &mut self.pool.get().unwrap())
    }

    pub fn user_exists(&self, username: &str) -> bool {
        user::username_exists(username, &mut self.pool.get().unwrap())
    }

    pub fn decrypt_secret(&self, secret: &str) -> Result<String, UserError> {
        secret::decrypt_string(secret, &self.config.encryption_key)
            .map_err(|e| UserError::SecretError(e.to_string()))
    }

    pub async fn login_with_github(
        &self,
        auth_token: &str,
        user_to_verify: &GithubUser,
    ) -> Result<(String, String), UserError> {
        log::info!("Attempting logging in with Github");
        let (client_id, client_secret) = self.get_github_credentials()?;
        log::debug!("Verifying user token");
        let _user_info =
            verify_user_token(&client_id, &client_secret, auth_token, user_to_verify).await?;

        let github_user = fetch_github_user(auth_token).await?;

        let conn = &mut self.pool.get().unwrap();
        let login_session =
            user::login_with_github(&github_user, conn, &self.config.encryption_key)?;

        self.tokens_manager
            .generate_login_token_pairs(&login_session, conn)
    }

    fn get_github_credentials(&self) -> Result<(String, String), UserError> {
        let client_id = self.config.github_client_id.clone().unwrap_or_default();

        let client_secret = self.config.github_client_secret.clone().unwrap_or_default();

        Ok((client_id, client_secret))
    }
}

impl Clone for AccountService {
    fn clone(&self) -> Self {
        AccountService {
            pool: self.pool.clone(),
            config: self.config.clone(),
            tokens_manager: tokens_manager::JWTTokensManager::new(
                &self.config.encryption_key.clone(),
                self.config.jwt_expiration,
                self.config.jwt_refresh_expiration,
            ),
        }
    }
}
