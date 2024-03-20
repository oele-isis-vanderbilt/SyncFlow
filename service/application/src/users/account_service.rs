use domain::models::{KeySecretPair, NewKeySecretPair};
use infrastructure::DbPool;
use jsonwebtoken::errors as jwt_errors;
use jsonwebtoken::TokenData;
use log::info;
use shared::deployment_config::DeploymentConfig;
use shared::user_models::LoginRequest;
use std::sync::Arc;

use super::{secrets, token, user};

pub struct AccountService {
    pool: Arc<DbPool>,
    config: DeploymentConfig,
    tokens_manager: token::JWTImplementation,
}

impl AccountService {
    pub fn new(pool: Arc<DbPool>, config: DeploymentConfig) -> Self {
        let secret = config.jwt_secret.clone();
        AccountService {
            pool,
            config,
            tokens_manager: token::JWTImplementation::new(&secret),
        }
    }

    /// Logs in a user
    pub fn login(&self, request: LoginRequest) -> Result<String, user::UserError> {
        let session_info_result = user::login(request, &mut self.pool.get().unwrap());
        match session_info_result {
            Ok(session_info) => {
                let user_secret_result =
                    user::find_user_secret(&session_info.user_id, &mut self.pool.get().unwrap());
                if let Err(_) = user_secret_result {
                    let key_secret_pair =
                        secrets::generate_key_pair_from_hex_key(&self.config.encryption_key);
                    key_secret_pair
                        .map_err(|e| user::UserError::EncryptionError(e.to_string()))
                        .and_then(|(key, secret)| {
                            self.populate_key_secret_pair(&session_info.user_id, &key, &secret)
                        })
                }
                let token = self.tokens_manager.generate_jwt_token(&session_info);
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
        let decoded_token = self.tokens_manager.decode_token(token.to_string());
        match decoded_token {
            Ok(token) => {
                let session_id = token.claims.login_session;
                if user::is_valid_login_session(&session_id, &mut self.pool.get().unwrap()) {
                    user::delete_login_session(&session_id, &mut self.pool.get().unwrap())
                        .map(|_| ())
                } else {
                    Err(user::UserError::LoginSessionNotFound(
                        "Login session not found".to_string(),
                    ))
                }
            }
            Err(e) => Err(user::UserError::TokenError(e.to_string())),
        }
    }

    pub fn get_pool(&self) -> Arc<DbPool> {
        self.pool.clone()
    }

    pub fn decode_token(
        &self,
        token: String,
    ) -> Result<TokenData<token::UserToken>, jwt_errors::Error> {
        let api_key = self.tokens_manager.get_api_key_from_token(token);
        match api_key {
            Some(key) => {
                let user_secret_key_pair =
                    user::find_user_secret(&key, &mut self.pool.get().unwrap());
            }
            None => {
                return Err(jwt_errors::Error::InvalidToken);
            }
        }
    }

    pub fn verify_token(&self, token_data: &TokenData<token::UserToken>) -> Result<String, String> {
        self.tokens_manager
            .verify_token(token_data, &mut self.pool.get().unwrap())
    }

    pub fn generate_jwt_token(
        &self,
        login_session_info: &user::LoginSessionInfo,
    ) -> Result<String, jwt_errors::Error> {
        self.tokens_manager.generate_jwt_token(login_session_info)
    }

    pub fn populate_key_secret_pair(
        &self,
        user_id: &i32,
        key: &String,
        secret: &String,
    ) -> Result<KeySecretPair, user::UserError> {
        let new_key_secret_pair = NewKeySecretPair {
            user_id: *user_id,
            api_key: key.clone(),
            secret: secret.clone(),
        };
        user::populate_key_secret_pair(new_key_secret_pair, &mut self.pool.get().unwrap())
    }
}

impl Clone for AccountService {
    fn clone(&self) -> Self {
        AccountService {
            pool: self.pool.clone(),
            config: self.config.clone(),
            tokens_manager: token::JWTImplementation::new(&self.config.jwt_secret),
        }
    }
}
