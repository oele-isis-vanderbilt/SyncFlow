use livekit_api::services::room::{CreateRoomOptions, RoomClient};
use livekit_api::services::ServiceResult;
use livekit_protocol::{self as proto, AutoTrackEgress, RoomEgress, S3Upload};
use shared::deployment_config::StorageConfig;
use shared::livekit_models::RoomOptions;

#[derive(Debug)]
pub struct RoomService {
    client: RoomClient,
    server_url: String,
    api_key: String,
    api_secret: String,
    storage_config: StorageConfig,
}

impl RoomService {
    pub fn new(
        server_url: String,
        api_key: String,
        api_secret: String,
        storage_config: StorageConfig,
    ) -> Self {
        let server_url = server_url.to_string().replace("ws", "http");

        Self {
            client: RoomClient::with_api_key(&server_url, &api_key, &api_secret),
            server_url,
            api_key,
            api_secret,
            storage_config,
        }
    }

    pub async fn create_room(
        &self,
        name: &str,
        options: RoomOptions,
    ) -> ServiceResult<proto::Room> {
        let create_options = CreateRoomOptions {
            max_participants: options.max_participants,
            metadata: options.metadata,
            empty_timeout: options.empty_timeout,
            egress: self.get_auto_egress(options.auto_recording),
            ..Default::default()
        };

        self.client.create_room(name, create_options).await
    }

    pub async fn delete_room(&self, name: &str) -> ServiceResult<()> {
        self.client.delete_room(name).await
    }

    pub async fn list_rooms(&self, names: Option<Vec<String>>) -> ServiceResult<Vec<proto::Room>> {
        let room_names = names.unwrap_or_default();
        self.client.list_rooms(room_names).await
    }

    pub async fn list_participants(
        &self,
        room_name: &str,
    ) -> ServiceResult<Vec<proto::ParticipantInfo>> {
        self.client.list_participants(room_name).await
    }

    pub async fn list_tracks(&self, room_name: &str) -> ServiceResult<Vec<proto::TrackInfo>> {
        self.client
            .list_participants(room_name)
            .await
            .map(|participants| participants.iter().flat_map(|p| p.tracks.clone()).collect())
    }

    fn get_auto_egress(&self, enabled: bool) -> Option<RoomEgress> {
        if enabled {
            match &self.storage_config {
                StorageConfig::Local(local_config) => Some(RoomEgress {
                    tracks: Some(AutoTrackEgress {
                        filepath: format!(
                            "{}/{}/tracks/{}/{}/{}-{}-{}-{}",
                            local_config.recording_root_path,
                            "{room_name}",
                            "{publisher_identity}",
                            "{time}",
                            "{track_type}",
                            "{track_source}",
                            "{track_id}",
                            "{time}"
                        ),
                        output: None,
                        disable_manifest: false,
                    }),
                    ..Default::default()
                }),
                StorageConfig::S3(s3_config) => Some(RoomEgress {
                    tracks: Some(AutoTrackEgress {
                        filepath: format!(
                            "{}/tracks/{}/{}/{}-{}-{}-{}",
                            "{room_name}",
                            "{publisher_identity}",
                            "{time}",
                            "{track_type}",
                            "{track_source}",
                            "{track_id}",
                            "{time}"
                        ),
                        output: Some(livekit_protocol::auto_track_egress::Output::S3({
                            S3Upload {
                                bucket: s3_config.bucket.clone(),
                                region: s3_config.region.clone(),
                                access_key: s3_config.access_key.clone(),
                                secret: s3_config.secret_key.clone(),
                                endpoint: s3_config.endpoint.clone(),
                                force_path_style: true,
                                ..Default::default()
                            }
                        })),
                        disable_manifest: false,
                    }),
                    ..Default::default()
                }),
            }
        } else {
            None
        }
    }
}

impl Clone for RoomService {
    fn clone(&self) -> Self {
        Self {
            client: RoomClient::with_api_key(&self.server_url, &self.api_key, &self.api_secret),
            server_url: self.server_url.clone(),
            api_key: self.api_key.clone(),
            api_secret: self.api_secret.clone(),
            storage_config: self.storage_config.clone(),
        }
    }
}
