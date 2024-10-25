use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TokenTypes {
    LoginToken(LoginToken),
    ApiToken(ApiToken),
    RefreshToken(RefreshToken),
    ProjectToken(ProjectToken),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectToken {
    pub iat: usize,
    pub exp: usize,
    pub iss: String,

    // Data
    pub project_id: String,
}

impl ProjectToken {
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        self.exp < now
    }
}
