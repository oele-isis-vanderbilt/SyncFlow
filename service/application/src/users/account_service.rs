use infrastructure::DbPool;
use shared::user_models::LoginRequest;
use std::sync::Arc;

use super::{token, user};

pub struct AccountService {
    pool: Arc<DbPool>,
}

impl AccountService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        AccountService { pool }
    }

    /// Logs in a user
    pub fn login(&self, request: LoginRequest) -> Result<String, String> {
        let session_info_result = user::login(request, &mut self.pool.get().unwrap());
        match session_info_result {
            Ok(session_info) => {
                let token = token::generate_jwt_token(&session_info);
                match token {
                    Ok(t) => Ok(t),
                    Err(e) => {
                        user::delete_login_session(
                            &session_info.session_id,
                            &mut self.pool.get().unwrap(),
                        );
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Logs out a user
    pub fn logout(&self, token: String) -> Result<(), String> {
        let decoded_token = token::decode_token(token.to_string());
        match decoded_token {
            Ok(token) => {
                let session_id = token.login_session;
                if user::is_valid_login_session(&session_id, &mut self.pool.get().unwrap()) {
                    user::delete_login_session(&session_id, &mut self.pool.get().unwrap())
                        .map(|_| ())
                        .map_err(|e| e.to_string())
                } else {
                    Err("Invalid session".to_string())
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl Clone for AccountService {
    fn clone(&self) -> Self {
        AccountService {
            pool: self.pool.clone(),
        }
    }
}
