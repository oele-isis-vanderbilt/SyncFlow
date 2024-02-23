use dotenv::dotenv;
use livekit_api::services::room::{CreateRoomOptions, RoomClient};
use livekit_api::services::ServiceResult;
use livekit_protocol as proto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct CreateRoomRequest {
    pub name: String,
    #[serde(default)]
    pub options: RoomOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct RoomOptions {
    pub empty_timeout: u32,
    pub max_participants: u32,
    pub metadata: String,
}

impl Default for RoomOptions {
    fn default() -> Self {
        Self {
            empty_timeout: 10 * 60,
            max_participants: 100,
            metadata: "".into(),
        }
    }
}

pub struct RoomService {
    client: RoomClient,
}

impl RoomService {
    pub fn new() -> Self {
        dotenv().ok();
        let server_url = std::env::var("LIVEKIT_SERVER_URL")
            .expect("LIVEKIT_SERVER_URL must be set")
            .replace("ws", "http");
        let api_key = std::env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY must be set");
        let api_secret = std::env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_KEY must be set");
        Self {
            client: RoomClient::with_api_key(&server_url, &api_key, &api_secret),
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
}
