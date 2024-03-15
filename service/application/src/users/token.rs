use crate::users::user::LoginSessionInfo;
use diesel::PgConnection;
use jsonwebtoken::errors as jwt_errors;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserToken {
    pub iat: usize,
    pub exp: usize,
    // Data
    pub user_name: String,
    pub user_id: i32,
    pub login_session: String,
}

pub type UserTokenType = TokenData<UserToken>;

pub struct JWTImplementation {
    pub jwt_secret: String,
}

impl JWTImplementation {
    pub fn new(jwt_secret: &str) -> Self {
        JWTImplementation {
            jwt_secret: jwt_secret.to_string(),
        }
    }

    pub fn generate_jwt_token(
        &self,
        login_session_info: &LoginSessionInfo,
    ) -> jwt_errors::Result<String> {
        let claims = UserToken {
            exp: 10000000000,
            iat: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
            user_name: login_session_info.user_name.clone(),
            user_id: login_session_info.user_id,
            login_session: login_session_info.session_id.clone(),
        };

        let secret = self.jwt_secret.clone();

        encode(
            &Header {
                alg: Algorithm::HS256,
                ..Header::default()
            },
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn decode_token(&self, token: String) -> jwt_errors::Result<TokenData<UserToken>> {
        let secret = self.jwt_secret.clone();
        decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
    }

    pub fn verify_token(
        &self,
        token_data: &TokenData<UserToken>,
        conn: &mut PgConnection,
    ) -> Result<String, String> {
        let claims = &token_data.claims;
        let session_id = &claims.login_session;
        if super::user::is_valid_login_session(session_id, conn) {
            Ok("Valid session".to_string())
        } else {
            Err("Invalid session".to_string())
        }
    }
}
