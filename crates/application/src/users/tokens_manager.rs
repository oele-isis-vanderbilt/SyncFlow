use std::cell::Ref;

use crate::users::secret::decrypt_string;
use crate::users::signed_token::{decode_jwt_unsafe, generate_and_sign_jwt, verify_and_decode_jwt};
use crate::users::user;
use crate::users::user::{LoginSessionInfo, UserError};
use diesel::PgConnection;
use domain::models::{ApiKey, Role, User};
use serde::{Deserialize, Serialize};

pub type UserTokenType = TokenTypes;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: i32,
    pub user_name: String,
    pub user_role: Role,
    pub login_session: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TokenTypes {
    LoginToken(LoginToken),
    ApiToken(ApiToken),
    RefreshToken(RefreshToken),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginToken {
    pub iat: usize,
    pub exp: usize,
    pub iss: String,

    // Data
    pub user_name: String,
    pub user_id: i32,
    pub role: Role,
    pub login_session: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub iat: usize,
    pub exp: usize,
    pub iss: String,

    // Data
    pub login_session: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiToken {
    pub iat: usize,
    pub exp: usize,
    pub iss: String,

    // Data
    pub project: Option<String>,
}

pub struct JWTTokensManager {
    pub encryption_key: String,
}

impl JWTTokensManager {
    pub fn new(encryption_key: String) -> Self {
        JWTTokensManager { encryption_key }
    }

    pub fn generate_login_token(
        &self,
        login_session_info: &LoginSessionInfo,
        conn: &mut PgConnection,
    ) -> Result<String, UserError> {
        let api_key = user::fetch_login_key(login_session_info.user_id, conn)?;
        let decrypted_secret = self.decrypt_user_secret(&api_key)?;

        let exp = chrono::Utc::now().timestamp() as usize + 60 * 60 * 24 * 1; // 1 day

        let user_token = LoginToken {
            iat: chrono::Utc::now().timestamp() as usize,
            exp,
            iss: api_key.key.to_owned(),
            user_name: login_session_info.user_name.to_owned(),
            user_id: login_session_info.user_id,
            role: login_session_info.user_role.to_owned(),
            login_session: login_session_info.session_id.to_owned(),
        };

        generate_and_sign_jwt::<LoginToken>(&user_token, &decrypted_secret)
            .map_err(|e| UserError::TokenError(e.to_string()))
    }

    pub fn generate_login_token_pairs(
        &self,
        login_session_info: &LoginSessionInfo,
        conn: &mut PgConnection,
    ) -> Result<(String, String), UserError> {
        let api_key = user::fetch_login_key(login_session_info.user_id, conn)?;
        let decrypted_secret = self.decrypt_user_secret(&api_key)?;

        //ToDo: Make this configurable
        let login_token_expiry = chrono::Utc::now().timestamp() as usize + 60 * 60 * 24 * 1; // 1 day
        let refresh_token_expiry = login_token_expiry + 60 * 60 * 24 * 6; // 7 days

        let user_token = LoginToken {
            iat: chrono::Utc::now().timestamp() as usize,
            exp: login_token_expiry,
            iss: api_key.key.to_owned(),
            user_name: login_session_info.user_name.to_owned(),
            user_id: login_session_info.user_id,
            role: login_session_info.user_role.to_owned(),
            login_session: login_session_info.session_id.to_owned(),
        };

        let refresh_token: RefreshToken = RefreshToken {
            iat: chrono::Utc::now().timestamp() as usize,
            exp: refresh_token_expiry,
            iss: api_key.key.to_owned(),
            login_session: login_session_info.session_id.to_owned(),
        };

        let login_token = generate_and_sign_jwt::<LoginToken>(&user_token, &decrypted_secret)
            .map_err(|e| UserError::TokenError(e.to_string()))?;

        let refresh_token =
            generate_and_sign_jwt::<RefreshToken>(&refresh_token, &decrypted_secret)
                .map_err(|e| UserError::TokenError(e.to_string()))?;

        Ok((login_token, refresh_token))
    }

    fn decrypt_user_secret(&self, api_key: &ApiKey) -> Result<String, UserError> {
        let encrypted_secret = &api_key.secret;
        decrypt_string(encrypted_secret, &self.encryption_key)
            .map_err(|e| UserError::SecretError(e.to_string()))
    }

    pub fn decode_token_unsafe(&self, token: &str) -> Result<TokenTypes, UserError> {
        if let Ok(token_data) = decode_jwt_unsafe::<LoginToken>(token) {
            return Ok(TokenTypes::LoginToken(token_data));
        } else if let Ok(token_data) = decode_jwt_unsafe::<RefreshToken>(token) {
            return Ok(TokenTypes::RefreshToken(token_data));
        } else if let Ok(token_data) = decode_jwt_unsafe::<ApiToken>(token) {
            return Ok(TokenTypes::ApiToken(token_data));
        } else {
            return Err(UserError::TokenError("Invalid token".to_string()));
        }
    }

    pub fn verify_token(
        &self,
        token: &str,
        conn: &mut PgConnection,
    ) -> Result<UserInfo, UserError> {
        let parsed_token = self.decode_token_unsafe(token)?;
        match parsed_token {
            TokenTypes::LoginToken(token_data) => {
                let api_key = user::fetch_api_key_by_id(token_data.iss.as_str(), conn)?;
                let encrypted_secret = api_key.secret;
                let decrypted_secret = decrypt_string(&encrypted_secret, &self.encryption_key)
                    .map_err(|e| UserError::SecretError(e.to_string()))?;
                let token_data = verify_and_decode_jwt::<LoginToken>(token, &decrypted_secret)
                    .map_err(|e| UserError::TokenError(e.to_string()))?;

                // Verify that the token is valid
                if token_data.iss != api_key.key {
                    return Err(UserError::TokenError("Invalid token".to_string()));
                }

                if !self.is_login_token_valid(&token_data, conn) {
                    return Err(UserError::TokenError("Invalid token".to_string()));
                }

                Ok(UserInfo {
                    user_id: token_data.user_id,
                    user_name: token_data.user_name.to_owned(),
                    user_role: token_data.role.to_owned(),
                    login_session: Some(token_data.login_session.to_owned()),
                })
            }

            TokenTypes::RefreshToken(token_data) => {
                let api_key = user::fetch_api_key_by_id(token_data.iss.as_str(), conn)?;
                let encrypted_secret = api_key.secret;
                let decrypted_secret = decrypt_string(&encrypted_secret, &self.encryption_key)
                    .map_err(|e| UserError::SecretError(e.to_string()))?;
                let token_data = verify_and_decode_jwt::<RefreshToken>(token, &decrypted_secret)
                    .map_err(|e| UserError::TokenError(e.to_string()))?;

                if token_data.iss != api_key.key {
                    return Err(UserError::TokenError("Invalid token".to_string()));
                }

                if !self.is_refresh_token_valid(&token_data, conn) {
                    return Err(UserError::TokenError("Invalid token".to_string()));
                }

                let user = user::get_user(api_key.user_id, conn)?;

                Ok(UserInfo {
                    user_id: user.id,
                    user_name: user.username.to_owned(),
                    user_role: user.role.to_owned(),
                    login_session: Some(token_data.login_session.to_owned()),
                })
            }

            TokenTypes::ApiToken(token_data) => {
                let api_key = user::fetch_api_key_by_key(token_data.iss.as_str(), conn)?;
                let encrypted_secret = api_key.secret;
                let decrypted_secret = decrypt_string(&encrypted_secret, &self.encryption_key)
                    .map_err(|e| UserError::SecretError(e.to_string()))?;
                let token_data = verify_and_decode_jwt::<ApiToken>(token, &decrypted_secret)
                    .map_err(|e| UserError::TokenError(e.to_string()))?;

                // Verify that the token is valid
                if token_data.iss != api_key.key {
                    return Err(UserError::TokenError("Invalid token".to_string()));
                }

                if !self.is_api_token_valid(&token_data) {
                    return Err(UserError::TokenError("Invalid token".to_string()));
                }

                let user = user::get_user(api_key.user_id, conn)?;
                Ok(UserInfo {
                    user_id: user.id,
                    user_name: user.username.to_owned(),
                    user_role: user.role.to_owned(),
                    login_session: None,
                })
            }
        }
    }

    pub fn is_login_token_valid(&self, token_data: &LoginToken, conn: &mut PgConnection) -> bool {
        let login_session_id = token_data.login_session.as_str();

        if !user::is_valid_login_session(login_session_id, conn) {
            return false;
        }

        // Verify that the token is not expired
        if token_data.exp < (chrono::Utc::now().timestamp() as usize) {
            return false;
        }

        true
    }

    pub fn is_refresh_token_valid(
        &self,
        token_data: &RefreshToken,
        conn: &mut PgConnection,
    ) -> bool {
        let login_session_id = token_data.login_session.as_str();

        if !user::is_valid_login_session(login_session_id, conn) {
            return false;
        }

        true
    }

    pub fn is_api_token_valid(&self, token_data: &ApiToken) -> bool {
        // Verify that the token is not expired
        if token_data.exp < (chrono::Utc::now().timestamp() as usize) {
            return false;
        }

        true
    }
}
