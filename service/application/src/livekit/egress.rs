use livekit_api::services::egress::{
    EgressClient, EgressListFilter, EgressListOptions, TrackEgressOutput,
};
use livekit_api::services::ServiceResult;

use livekit_protocol::{DirectFileOutput, EgressInfo, S3Upload};
use shared::deployment_config::StorageConfig;
use std::collections::HashMap;

#[derive(Debug)]
pub struct EgressService {
    client: EgressClient,
    server_url: String,
    api_key: String,
    api_secret: String,
    storage_config: StorageConfig,
}

impl EgressService {
    pub fn new(
        server_url: String,
        api_key: String,
        api_secret: String,
        storage_config: StorageConfig,
    ) -> Self {
        let server_url = server_url.to_string().replace("ws", "http");

        Self {
            client: EgressClient::with_api_key(&server_url, &api_key, &api_secret),
            server_url,
            api_key,
            api_secret,
            storage_config,
        }
    }

    pub async fn list_egresses(&self, room_name: &str) -> ServiceResult<Vec<EgressInfo>> {
        let options = EgressListOptions {
            active: false,
            filter: EgressListFilter::Room(room_name.into()),
        };

        self.client.list_egress(options).await
    }

    pub async fn start_local_track_egress(
        &self,
        room_name: &str,
        track_sid: &str,
    ) -> ServiceResult<EgressInfo> {
        match &self.storage_config {
            StorageConfig::Local(ref local_config) => {
                let output = TrackEgressOutput::File(Box::new(DirectFileOutput {
                    filepath: format!(
                        "{}/{}/tracks/{}/{}-{}-{}-{}",
                        local_config.recording_root_path,
                        room_name,
                        "{publisher_identity}",
                        "{track_type}",
                        "{track_source}",
                        "{track_id}",
                        "{time}"
                    ),
                    output: None,
                    disable_manifest: false,
                }));
                self.client
                    .start_track_egress(room_name, output, track_sid)
                    .await
            }
            StorageConfig::S3(s3_config) => {
                let output = TrackEgressOutput::File(Box::new(DirectFileOutput {
                    filepath: format!(
                        "{}/tracks/{}/{}-{}-{}-{}",
                        room_name,
                        "{publisher_identity}",
                        "{track_type}",
                        "{track_source}",
                        "{track_id}",
                        "{time}"
                    ),
                    output: Some(livekit_protocol::direct_file_output::Output::S3(S3Upload {
                        bucket: s3_config.bucket.clone(),
                        region: s3_config.region.clone(),
                        access_key: s3_config.access_key.clone(),
                        secret: s3_config.secret_key.clone(),
                        endpoint: s3_config.endpoint.clone(),
                        tagging: "".to_string(),
                        force_path_style: true,
                        content_disposition: "".to_string(),
                        metadata: HashMap::new(),
                    })),
                    disable_manifest: false,
                }));
                self.client
                    .start_track_egress(room_name, output, track_sid)
                    .await
            }
        }
    }

    pub async fn stop_egress(&self, egress_id: &str) -> ServiceResult<EgressInfo> {
        self.client.stop_egress(egress_id).await
    }
}

impl Clone for EgressService {
    fn clone(&self) -> Self {
        Self {
            client: EgressClient::with_api_key(&self.server_url, &self.api_key, &self.api_secret),
            server_url: self.server_url.clone(),
            api_key: self.api_key.clone(),
            api_secret: self.api_secret.clone(),
            storage_config: self.storage_config.clone(),
        }
    }
}
