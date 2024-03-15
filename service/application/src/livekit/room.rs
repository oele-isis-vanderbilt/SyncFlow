use livekit_api::services::room::{CreateRoomOptions, RoomClient};
use livekit_api::services::ServiceResult;
use livekit_protocol as proto;
use shared::livekit_models::RoomOptions;

#[derive(Debug)]
pub struct RoomService {
    client: RoomClient,
    server_url: String,
    api_key: String,
    api_secret: String,
}

impl RoomService {
    pub fn new(server_url: String, api_key: String, api_secret: String) -> Self {
        let server_url = server_url.to_string().replace("ws", "http");

        Self {
            client: RoomClient::with_api_key(&server_url, &api_key, &api_secret),
            server_url,
            api_key,
            api_secret,
        }
    }

    pub async fn create_room(
        &self,
        name: &str,
        options: RoomOptions,
    ) -> ServiceResult<proto::Room> {
        let mut create_options = CreateRoomOptions::default();
        create_options.max_participants = options.max_participants;
        create_options.metadata = options.metadata;
        create_options.empty_timeout = options.empty_timeout;

        self.client.create_room(name, create_options).await
    }

    pub async fn delete_room(&self, name: &str) -> ServiceResult<()> {
        self.client.delete_room(name).await
    }

    pub async fn list_rooms(&self, names: Option<Vec<String>>) -> ServiceResult<Vec<proto::Room>> {
        let room_names = names.unwrap_or_default();
        self.client.list_rooms(room_names).await
    }
}

impl Clone for RoomService {
    fn clone(&self) -> Self {
        Self {
            client: RoomClient::with_api_key(&self.server_url, &self.api_key, &self.api_secret),
            server_url: self.server_url.clone(),
            api_key: self.api_key.clone(),
            api_secret: self.api_secret.clone(),
        }
    }
}
