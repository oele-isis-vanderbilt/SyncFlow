use super::{secret, tokens_manager, user};
use crate::users::tokens_manager::{TokenTypes, UserInfo};
use crate::users::user::UserError;
use domain::models::{ApiKey, User};
use infrastructure::DbPool;
use shared::deployment_config::DeploymentConfig;
use shared::user_models::LoginRequest;
use shared::user_models::{ApiKeyResponseWithoutSecret, RefreshTokenRequest};
use std::sync::Arc;

pub struct AccountService {
    pool: Arc<DbPool>,
    config: DeploymentConfig,
    tokens_manager: tokens_manager::JWTTokensManager,
}

impl AccountService {
    pub fn new(pool: Arc<DbPool>, config: DeploymentConfig) -> Self {
        let encryption_key = config.encryption_key.clone();
        AccountService {
            pool,
            config,
            tokens_manager: tokens_manager::JWTTokensManager::new(encryption_key),
        }
    }

    /// Logs in a user
    pub fn login(&self, request: LoginRequest) -> Result<(String, String), UserError> {
        let session_info_result = user::login(
            request,
            &mut self.pool.get().unwrap(),
            &self.config.encryption_key,
        );
        match session_info_result {
            Ok(session_info) => {
                let conn = &mut self.pool.get().unwrap();
                let token = self
                    .tokens_manager
                    .generate_login_token_pairs(&session_info, conn);
                match token {
                    Ok(t) => Ok(t),
                    Err(e) => {
                        let _ = user::delete_login_session(
                            &session_info.session_id,
                            &mut self.pool.get().unwrap(),
                        );
                        Err(user::UserError::TokenError(e.to_string()))
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn refresh_token(
        &self,
        request: RefreshTokenRequest,
    ) -> Result<(String, String), UserError> {
        let user_info = self
            .tokens_manager
            .verify_token(&request.refresh_token, &mut self.pool.get().unwrap())?;
        let sid = user_info
            .login_session
            .ok_or(UserError::LoginSessionNotFound(
                "No login session found for the refresh token.".to_string(),
            ))?;
        let conn = &mut self.pool.get().unwrap();
        let login_session_info = user::get_login_session_info(user_info.user_id, &sid, conn)?;

        self.tokens_manager
            .generate_login_token_pairs(&login_session_info, conn)
    }

    /// Logs out a user
    pub fn logout(&self, token: &str) -> Result<(), user::UserError> {
        let conn = &mut self.pool.get().unwrap();
        let decoded_token = self.tokens_manager.verify_token(token, conn);
        match decoded_token {
            Ok(token) => {
                let session_id = token.login_session;
                if let Some(session_id) = session_id {
                    let _ = user::delete_login_session(&session_id, conn);
                } else {
                    return Err(UserError::TokenError("Invalid token".to_string()));
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
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
        comments: Option<String>,
    ) -> Result<ApiKey, UserError> {
        user::generate_non_login_api_key(
            user_id,
            &self.config.encryption_key,
            comments,
            &mut self.pool.get().unwrap(),
        )
    }

    pub fn list_api_keys(
        &self,
        user_id: i32,
    ) -> Result<Vec<ApiKeyResponseWithoutSecret>, UserError> {
        let mut conn = self.pool.get().unwrap();
        user::get_all_api_keys(user_id, &mut conn).map(|keys| {
            keys.into_iter()
                .map(|api_key| ApiKeyResponseWithoutSecret {
                    key: api_key.key.clone(),
                    comment: api_key.comment.unwrap_or_default(),
                    created_at: api_key
                        .created_at
                        .map(|c| c.and_utc().timestamp() as usize)
                        .unwrap_or_default(),
                })
                .collect::<Vec<ApiKeyResponseWithoutSecret>>()
        })
    }

    pub fn delete_api_key(
        &self,
        user_id: i32,
        key_id: &str,
    ) -> Result<ApiKeyResponseWithoutSecret, UserError> {
        user::delete_api_key(user_id, key_id, &mut self.pool.get().unwrap()).map(|api_key| {
            ApiKeyResponseWithoutSecret {
                key: api_key.key.to_owned(),
                comment: api_key.comment.unwrap_or_default(),
                created_at: api_key
                    .created_at
                    .map(|c| c.and_utc().timestamp() as usize)
                    .unwrap_or_default(),
            }
        })
    }

    pub fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        admin: bool,
    ) -> Result<User, UserError> {
        user::create_user(
            username,
            email,
            password,
            admin,
            &mut self.pool.get().unwrap(),
        )
    }

    pub fn user_exists(&self, username: &str) -> bool {
        user::user_exists(username, &mut self.pool.get().unwrap())
    }

    pub fn decrypt_secret(&self, secret: &str) -> Result<String, UserError> {
        secret::decrypt_string(secret, &self.config.encryption_key)
            .map_err(|e| UserError::SecretError(e.to_string()))
    }
}

impl Clone for AccountService {
    fn clone(&self) -> Self {
        AccountService {
            pool: self.pool.clone(),
            config: self.config.clone(),
            tokens_manager: tokens_manager::JWTTokensManager::new(
                self.config.encryption_key.clone(),
            ),
        }
    }
}
