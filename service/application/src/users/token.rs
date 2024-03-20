use crate::users::user::LoginSessionInfo;
use diesel::PgConnection;
use domain::models::Role;
use jsonwebtoken::errors as jwt_errors;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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

pub type UserTokenType = TokenData<UserToken>;

pub struct JWTImplementation {}

impl JWTImplementation {
    pub fn new() -> Self {
        JWTImplementation {}
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
            iss: login_session_info.api_key.to_owned(),
            user_name: login_session_info.user_name.to_owned(),
            user_id: login_session_info.user_id,
            role: login_session_info.user_role.to_owned(),
            login_session: login_session_info.session_id.to_owned(),
        };

        let secret = login_session_info.jwt_secret.clone();

        encode(
            &Header {
                alg: Algorithm::HS256,
                ..Header::default()
            },
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn decode_token(
        &self,
        token: String,
        secret: String,
    ) -> jwt_errors::Result<TokenData<UserToken>> {
        decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
    }

    pub fn get_api_key_from_token(&self, token: String) -> Option<String> {
        let key = DecodingKey::from_secret(&[]);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.insecure_disable_signature_validation();
        let token_data = decode::<UserToken>(&token, &key, &validation);
        match token_data {
            Ok(token_data) => {
                let claims = token_data.claims;
                Some(claims.iss)
            }
            Err(_) => None,
        }
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
