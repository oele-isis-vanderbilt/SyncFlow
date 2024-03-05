use crate::users::user::LoginSessionInfo;
use diesel::PgConnection;
use jsonwebtoken::errors as jwt_errors;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: usize,
    pub exp: usize,
    // Data
    pub user_name: String,
    pub login_session: String,
}

pub fn generate_jwt_token(login_session_info: &LoginSessionInfo) -> Result<String, String> {
    let claims = UserToken {
        exp: 10000000000,
        iat: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
        user_name: login_session_info.user_name.clone(),
        login_session: login_session_info.session_id.clone(),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header {
            alg: Algorithm::HS256,
            ..Header::default()
        },
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| e.to_string())
}

pub fn decode_token(token: String) -> jwt_errors::Result<TokenData<UserToken>> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

pub fn verify_token(
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
