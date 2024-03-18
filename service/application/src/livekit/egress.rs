use livekit_api::services::egress::{EgressClient, EgressListFilter, EgressListOptions};
use livekit_api::services::ServiceResult;
use livekit_protocol::EgressInfo;
use shared::deployment_config::StorageConfig;

#[derive(Debug)]
pub struct EgressService {
    client: EgressClient,
    server_url: String,
    api_key: String,
    api_secret: String,
    storage_config: StorageConfig,
}

impl EgressService {
    pub fn new(server_url: String, api_key: String, api_secret: String) -> Self {
        let server_url = server_url.to_string().replace("ws", "http");

        Self {
            client: EgressClient::with_api_key(&server_url, &api_key, &api_secret),
            server_url,
            api_key,
            api_secret,
        }
    }

    pub async fn list_egresses(&self, room_name: &str) -> ServiceResult<Vec<EgressInfo>> {
        let options = EgressListOptions {
            active: false,
            filter: EgressListFilter::Room(room_name.into()),
        };

        self.client.list_egress(options).await
    }

    pub async fn start_track_egress(&self, room_name: &str, track_sid: &str) -> ServiceResult<EgressInfo> {


    }
}

impl Clone for EgressService {
    fn clone(&self) -> Self {
        Self {
            client: EgressClient::with_api_key(&self.server_url, &self.api_key, &self.api_secret),
            server_url: self.server_url.clone(),
            api_key: self.api_key.clone(),
            api_secret: self.api_secret.clone(),
        }
    }
}