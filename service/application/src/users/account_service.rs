use crate::users::tokens_manager::UserToken;
use crate::users::user::UserError;
use domain::models::ApiKey;
use infrastructure::DbPool;
use shared::deployment_config::DeploymentConfig;
use shared::user_models::LoginRequest;
use std::sync::Arc;

use super::{secret, tokens_manager, user};

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
    pub fn login(&self, request: LoginRequest) -> Result<String, user::UserError> {
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
                    .generate_login_token(&session_info, conn);
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

    /// Logs out a user
    pub fn logout(&self, token: &str) -> Result<(), user::UserError> {
        let conn = &mut self.pool.get().unwrap();
        let decoded_token = self.tokens_manager.verify_token(token, conn);
        match decoded_token {
            Ok(token) => {
                if self.tokens_manager.is_token_valid(&token, conn) {
                    let session_id = token.login_session;
                    let _ = user::delete_login_session(&session_id, conn);
                    Ok(())
                } else {
                    Err(UserError::TokenError("Invalid token".to_string()))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_pool(&self) -> Arc<DbPool> {
        self.pool.clone()
    }

    pub fn decode_token(&self, token: String) -> Result<UserToken, UserError> {
        self.tokens_manager.decode_token_unsafe(&token)
    }

    pub fn verify_token(&self, token_data: &str) -> Result<UserToken, UserError> {
        self.tokens_manager
            .verify_token(token_data, &mut self.pool.get().unwrap())
    }

    pub fn generate_login_token(
        &self,
        login_session_info: &user::LoginSessionInfo,
    ) -> Result<String, UserError> {
        self.tokens_manager
            .generate_login_token(login_session_info, &mut self.pool.get().unwrap())
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
