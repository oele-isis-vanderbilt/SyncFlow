use livekit_text_egress_actor::text_egress_actor::{
    EgressMessages, ListEgressMessages, TextEgressInfo, TextEgressRequest,
};
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use shared::deployment_config::S3Config;
use std::collections::HashMap;

pub struct TextEgressService {
    client: Client,
    base_url: String,
    s3_config: S3Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextEgressServiceError {
    ReqwestError(ReqwestError),
    TextEgressError(String),
}

impl TextEgressService {
    pub fn new(base_url: &str, s3_config: S3Config) -> Self {
        TextEgressService {
            client: Client::new(),
            base_url: base_url.to_string(),
            s3_config,
        }
    }

    pub async fn list_egresses(
        &self,
    ) -> Result<HashMap<String, TextEgressInfo>, TextEgressServiceError> {
        let url = format!("{}/egresses", self.base_url);
        let res = self.client.get(&url).send().await?;
        if !res.status().is_success() {
            return Err(TextEgressServiceError::TextEgressError(
                res.text()
                    .await
                    .map_err(TextEgressServiceError::ReqwestError)?,
            ));
        } else {
            let res = res
                .json::<HashMap<String, TextEgressInfo>>()
                .await
                .map_err(TextEgressServiceError::ReqwestError)?;
            Ok(res)
        }
    }

    pub async fn create_egress(
        &self,
        room_name: &str,
        topic: Option<String>,
    ) -> Result<TextEgressInfo, TextEgressServiceError> {
        let url = format!("{}/egress", self.base_url);

        let egress_request = TextEgressRequest {
            room_name: room_name.to_string(),
            topic,
            s3_bucket_name: self.s3_config.bucket.clone(),
            s3_access_key: self.s3_config.access_key.clone(),
            s3_secret_key: self.s3_config.secret_key.clone(),
            s3_region: self.s3_config.region.clone(),
            s3_endpoint: self.s3_config.endpoint.clone(),
        };

        let res = self.client.post(&url).json(&egress_request).send().await;
        if !res.status().is_success() {
            return Err(TextEgressServiceError::TextEgressError(
                res.text()
                    .await
                    .map_err(TextEgressServiceError::ReqwestError)?,
            ));
        } else {
            let res = res
                .json::<TextEgressInfo>()
                .await
                .map_err(TextEgressServiceError::ReqwestError)?;
            Ok(res)
        }
    }

    pub async fn get_egress(&self, egress_id: &str) {
        let url = format!("{}/egress/{}", self.base_url, egress_id);
        let res = self.client.get(&url).send().await;
        if !res.status().is_success() {
            return Err(TextEgressServiceError::TextEgressError(
                res.text()
                    .await
                    .map_err(TextEgressServiceError::ReqwestError)?,
            ));
        } else {
            let res = res
                .json::<TextEgressInfo>()
                .await
                .map_err(TextEgressServiceError::ReqwestError)?;
            Ok(res)
        }
    }

    pub async fn stop_egress(&self, egress_id: &str) {
        let url = format!("{}/stop-egress/{}", self.base_url, egress_id);
        let res = self.client.post(&url).send().await;
        if !res.status().is_success() {
            return Err(TextEgressServiceError::TextEgressError(
                res.text()
                    .await
                    .map_err(TextEgressServiceError::ReqwestError)?,
            ));
        } else {
            let res = res
                .json::<TextEgressInfo>()
                .await
                .map_err(TextEgressServiceError::ReqwestError)?;
            Ok(res)
        }
    }
}
