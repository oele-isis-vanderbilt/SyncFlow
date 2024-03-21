use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyRequest {
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyResponse {
    pub key: String,
    pub secret: String,
    pub comment: String,
}

impl TokenResponse {
    pub fn new(token: String, token_type: String) -> Self {
        Self { token, token_type }
    }

    pub fn bearer(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}
