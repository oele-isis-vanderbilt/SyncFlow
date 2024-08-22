use base64::{alphabet, engine, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::constants::APPLICATION_NAME;
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubTokenPayload {
    access_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct GithubUser {
    pub login: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Error)]
pub enum GithubOAuthError {
    #[error("Verification error: {0}")]
    VerificationError(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubVerificationResponse {
    pub app: GithubApp,
    pub user: GithubUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubApp {
    name: String,
    url: String,
    client_id: String,
}

pub async fn verify_user_token(
    client_id: &str,
    client_secret: &str,
    access_token: &str,
    user: &GithubUser,
) -> Result<GithubVerificationResponse, GithubOAuthError> {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::PAD);
    let credentials = engine.encode(format!("{}:{}", client_id, client_secret));
    let payload = GithubTokenPayload {
        access_token: access_token.to_owned(),
    };

    let client = Client::new();
    let verification_url = format!("https://api.github.com/applications/{}/token", client_id);

    let response = client
        .post(&verification_url)
        .header("Accept", "application/vnd.github+json")
        .header("X-Github-Api-Version", "2022-11-28")
        .header("Authorization", format!("Basic {}", credentials))
        .header(reqwest::header::USER_AGENT, APPLICATION_NAME)
        .json(&payload)
        .send()
        .await
        .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;

    if response.status().is_success() {
        let response_text = response
            .text()
            .await
            .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;

        let json_value: Value = serde_json::from_str(&response_text)
            .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;

        let mut verification_response: GithubVerificationResponse =
            serde_json::from_value(json_value)
                .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;

        if verification_response.app.client_id != client_id {
            let message =
                "The client ID in the response does not match the client ID in the request.";
            return Err(GithubOAuthError::VerificationError(message.to_owned()));
        }
        if verification_response.user.login != *user.login {
            let message = "The user in the response does not match the user in the request.";
            return Err(GithubOAuthError::VerificationError(message.to_owned()));
        }

        verification_response.user.email = user.email.clone();

        Ok(verification_response)
    } else {
        let error_text = response
            .text()
            .await
            .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;
        Err(GithubOAuthError::VerificationError(error_text))
    }
}

pub async fn fetch_github_user(token: &str) -> Result<GithubUser, GithubOAuthError> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("X-Github-Api-Version", "2022-11-28")
        .header("Accept", "application/vnd.github+json")
        .header(reqwest::header::USER_AGENT, APPLICATION_NAME)
        .send()
        .await
        .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;

    if response.status().is_success() {
        let response_text = response
            .text()
            .await
            .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;
        let github_user: GithubUser = serde_json::from_str(&response_text)
            .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;
        Ok(github_user)
    } else {
        let error_text = response
            .text()
            .await
            .map_err(|err| GithubOAuthError::ReqwestError(err.to_string()))?;
        Err(GithubOAuthError::VerificationError(error_text))
    }
}
