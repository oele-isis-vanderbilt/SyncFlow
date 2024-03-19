use livekit_api::services::egress::{
    EgressClient, EgressListFilter, EgressListOptions, EgressOutput, TrackEgressOutput,
};
use livekit_api::services::ServiceResult;
use livekit_protocol::track_egress_request::Output;
use livekit_protocol::{DirectFileOutput, EgressInfo, TrackEgressRequest};
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
        let output = TrackEgressOutput::File(
            Box::new(
                DirectFileOutput {
                    filepath: "/out/tracks/{room_name}/{publisher_identity}/{track_type}-{track_source}-{track_id}-{time}".to_string(),
                    output: None,
                    disable_manifest: false
            })
        );

        self.client
            .start_track_egress(room_name, output, track_sid)
            .await
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
