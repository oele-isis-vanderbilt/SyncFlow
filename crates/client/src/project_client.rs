use livekit_protocol::ParticipantInfo;
use reqwest::Client;
use shared::livekit_models::TokenResponse;
use shared::signed_token::SignedTokenError;
use shared::{
    claims::ProjectToken,
    device_models::{DeviceRegisterRequest, DeviceResponse},
    livekit_models::TokenRequest,
    project_models::{NewSessionRequest, ProjectSessionResponse, ProjectSummary},
    signed_token::{generate_and_sign_jwt, verify_and_decode_jwt},
    user_models::ProjectInfo,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectClientError {
    #[error("Signed token error: {0}")]
    TokenError(#[from] SignedTokenError),

    #[error("HTTP error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug)]
pub struct ProjectClient {
    base_url: String,
    api_key: String,
    api_secret: String,
    project_id: String,
    client: reqwest::Client,
    api_token: Mutex<Option<String>>,
}

impl ProjectClient {
    pub fn new(base_url: &str, project_id: &str, api_key: &str, api_secret: &str) -> Self {
        ProjectClient {
            base_url: base_url.to_string(),
            project_id: project_id.to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            client: Client::new(),
            api_token: Mutex::new(None),
        }
    }

    pub async fn get_api_token(&self) -> Result<String, ProjectClientError> {
        let mut token_lock: tokio::sync::MutexGuard<'_, Option<String>> =
            self.api_token.lock().await;
        if token_lock.is_none() {
            *token_lock = Some(self.generate_api_token()?);
        }

        let has_token_expired = self.is_expired(token_lock.as_ref().unwrap())?;

        if has_token_expired {
            *token_lock = Some(self.generate_api_token()?);
        }

        Ok(token_lock.as_ref().unwrap().clone())
    }

    pub fn generate_api_token(&self) -> Result<String, ProjectClientError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let claims = ProjectToken {
            iat: now,
            exp: now + 3600, // 1 hour expiration
            iss: self.api_key.clone(),
            project_id: self.project_id.clone(),
        };

        let token = generate_and_sign_jwt::<ProjectToken>(&claims, &self.api_secret)?;

        Ok(token)
    }

    pub fn is_expired(&self, token: &str) -> Result<bool, ProjectClientError> {
        let claims = verify_and_decode_jwt::<ProjectToken>(token, &self.api_secret)?;

        Ok(claims.is_expired())
    }

    pub async fn get_project_details(&self) -> Result<ProjectInfo, ProjectClientError> {
        let path = format!("projects/{}", self.project_id);

        self.authenticated_get(&path).await
    }

    pub async fn delete_project(&self) -> Result<ProjectInfo, ProjectClientError> {
        let path = format!("projects/{}", self.project_id);

        self.authenticated_delete(&path).await
    }

    pub async fn summarize_project(&self) -> Result<ProjectSummary, ProjectClientError> {
        let path = format!("projects/{}/summarize", self.project_id);

        self.authenticated_get(&path).await
    }

    pub async fn create_session(
        &self,
        new_session_request: &NewSessionRequest,
    ) -> Result<ProjectSessionResponse, ProjectClientError> {
        let path = format!("projects/{}/create-session", self.project_id);

        self.authenticated_post(&path, new_session_request).await
    }

    pub async fn get_sessions(&self) -> Result<Vec<ProjectSessionResponse>, ProjectClientError> {
        let path = format!("projects/{}/sessions", self.project_id);

        self.authenticated_get(&path).await
    }

    pub async fn get_session(
        &self,
        session_id: &str,
    ) -> Result<ProjectSessionResponse, ProjectClientError> {
        let path = format!("projects/{}/sessions/{}", self.project_id, session_id);

        self.authenticated_get(&path).await
    }

    pub async fn get_participants(
        &self,
        session_id: &str,
    ) -> Result<Vec<ParticipantInfo>, ProjectClientError> {
        let path = format!(
            "projects/{}/sessions/{}/participants",
            self.project_id, session_id
        );

        self.authenticated_get(&path).await
    }

    pub async fn get_livekit_session_info(
        &self,
        session_id: &str,
    ) -> Result<(), ProjectClientError> {
        let path = format!(
            "projects/{}/sessions/{}/livekit-session-info",
            self.project_id, session_id
        );

        self.authenticated_get(&path).await
    }

    pub async fn generate_session_token(
        &self,
        session_id: &str,
        token_request: &TokenRequest,
    ) -> Result<TokenResponse, ProjectClientError> {
        let path = format!("projects/{}/sessions/{}/token", self.project_id, session_id);

        self.authenticated_post(&path, token_request).await
    }

    pub async fn stop_session(
        &self,
        session_id: &str,
    ) -> Result<ProjectSessionResponse, ProjectClientError> {
        let path = format!("projects/{}/sessions/{}/stop", self.project_id, session_id);

        self.authenticated_post(&path, &()).await
    }

    pub async fn get_devices(&self) -> Result<Vec<DeviceResponse>, ProjectClientError> {
        let path = format!("projects/{}/devices", self.project_id);

        self.authenticated_get(&path).await
    }

    pub async fn get_device(&self, device_id: &str) -> Result<DeviceResponse, ProjectClientError> {
        let path = format!("projects/{}/devices/{}", self.project_id, device_id);

        self.authenticated_get(&path).await
    }

    pub async fn register_device(
        &self,
        device_register_request: &DeviceRegisterRequest,
    ) -> Result<DeviceResponse, ProjectClientError> {
        let path = format!("projects/{}/devices/register", self.project_id);

        self.authenticated_post(&path, device_register_request)
            .await
    }

    pub async fn delete_device(
        &self,
        device_id: &str,
    ) -> Result<DeviceResponse, ProjectClientError> {
        let path = format!("projects/{}/devices/{}", self.project_id, device_id);

        self.authenticated_delete(&path).await
    }

    pub async fn authenticated_get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, ProjectClientError> {
        let token = self.get_api_token().await?;
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "SyncFlow Project Client/ V0.1.0")
            .send()
            .await?;

        let response_json = response.json::<T>().await?;

        Ok(response_json)
    }

    pub async fn authenticated_post<T: serde::de::DeserializeOwned, E: serde::Serialize>(
        &self,
        path: &str,
        body: &E,
    ) -> Result<T, ProjectClientError> {
        let token = self.get_api_token().await?;
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .header("User-Agent", "SyncFlow Project Client/ V0.1.0")
            .json(body)
            .send()
            .await?;

        let response_json = response.json::<T>().await?;

        Ok(response_json)
    }

    pub async fn authenticated_delete<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, ProjectClientError> {
        let token = self.get_api_token().await?;
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "SyncFlow Project Client/ V0.1.0")
            .send()
            .await?;

        let response_json = response.json::<T>().await?;

        Ok(response_json)
    }
}
