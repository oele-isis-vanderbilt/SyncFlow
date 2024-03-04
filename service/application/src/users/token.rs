use crate::users::user::LoginSessionInfo;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
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

pub fn decode_token(token: String) -> Result<UserToken, String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}
