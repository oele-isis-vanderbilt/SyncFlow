use crate::users::secret::decrypt_string;
use crate::users::signed_token::{decode_jwt_unsafe, generate_and_sign_jwt, verify_and_decode_jwt};
use crate::users::user;
use crate::users::user::{LoginSessionInfo, UserError};
use diesel::PgConnection;
use domain::models::Role;
use serde::{Deserialize, Serialize};

pub type UserTokenType = UserToken;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserToken {
    pub iat: usize,
    pub exp: usize,
    pub iss: String,

    // Data
    pub user_name: String,
    pub user_id: i32,
    pub role: Role,
    pub login_session: String,
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
        let user_id = login_session_info.user_id;
        let api_key = user::fetch_login_key(user_id, conn)?;
        let encrypted_secret = api_key.secret;
        let decrypted_secret = decrypt_string(&encrypted_secret, &self.encryption_key)
            .map_err(|e| UserError::SecretError(e.to_string()))?;

        let exp = chrono::Utc::now().timestamp() as usize + 60 * 60 * 24 * 7; // 7 days

        let user_token = UserToken {
            iat: chrono::Utc::now().timestamp() as usize,
            exp,
            iss: api_key.key.to_owned(),
            user_name: login_session_info.user_name.to_owned(),
            user_id: login_session_info.user_id,
            role: login_session_info.user_role.to_owned(),
            login_session: login_session_info.session_id.to_owned(),
        };

        generate_and_sign_jwt::<UserToken>(&user_token, &decrypted_secret)
            .map_err(|e| UserError::TokenError(e.to_string()))
    }

    pub fn decode_token_unsafe(&self, token: &str) -> Result<UserToken, UserError> {
        let token_data = decode_jwt_unsafe::<UserToken>(token)
            .map_err(|e| UserError::TokenError(e.to_string()))?;
        Ok(token_data)
    }

    pub fn verify_token(
        &self,
        token: &str,
        conn: &mut PgConnection,
    ) -> Result<UserToken, UserError> {
        let token_data = self.decode_token_unsafe(token)?;
        let api_key = user::fetch_api_key_by_id(token_data.iss.as_str(), conn)?;
        let encrypted_secret = api_key.secret;
        let decrypted_secret = decrypt_string(&encrypted_secret, &self.encryption_key)
            .map_err(|e| UserError::SecretError(e.to_string()))?;
        let token_data = verify_and_decode_jwt::<UserToken>(token, &decrypted_secret)
            .map_err(|e| UserError::TokenError(e.to_string()))?;

        // Verify that the token is valid
        if token_data.iss != api_key.key {
            return Err(UserError::TokenError("Invalid token".to_string()));
        }

        Ok(token_data)
    }

    pub fn is_token_valid(&self, token_data: &UserToken, conn: &mut PgConnection) -> bool {
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
}
