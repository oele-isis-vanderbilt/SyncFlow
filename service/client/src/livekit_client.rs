use shared::{
    livekit_models::{
        CreateRoomRequest, RoomCreationResult, RoomOptions, TokenRequest, TokenResponse,
        VideoGrantsWrapper,
    },
    response_models::Response as ApiResponse,
};

use crate::http_client::{HTTPAuthTokenClient, JSONResult};

#[derive(PartialEq, Clone, Debug)]
pub enum TokenGeneratePermissions {
    Publish,
    Subscribe,
    PublishSubscribe,
}

pub struct LiveKitClient {
    http_client: HTTPAuthTokenClient,
}

impl LiveKitClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let http_client = HTTPAuthTokenClient::new(base_url, token);
        LiveKitClient { http_client }
    }

    pub fn healthcheck(&self) -> JSONResult<ApiResponse> {
        let healthcheck = self.http_client.get("livekit/health");
        self.http_client.map_response::<ApiResponse>(healthcheck)
    }

    pub fn generate_token(
        &self,
        identity: &str,
        room_name: &str,
        token_generate_permissions: Option<TokenGeneratePermissions>,
        room_record: Option<bool>,
        can_create_room: Option<bool>,
    ) -> JSONResult<TokenResponse> {
        let room_permission =
            token_generate_permissions.unwrap_or(TokenGeneratePermissions::Publish);
        let can_create_room = can_create_room.unwrap_or(false);
        let can_record = room_record.unwrap_or(false);

        // ToDo: Add support for a token request builder
        let token_request = TokenRequest {
            identity: identity.to_string(),
            video_grants: VideoGrantsWrapper {
                room_create: can_create_room,
                room_list: true,
                room_record: can_record,
                room: room_name.to_string(),
                can_publish: room_permission == TokenGeneratePermissions::Publish
                    || room_permission == TokenGeneratePermissions::PublishSubscribe,
                can_subscribe: room_permission == TokenGeneratePermissions::Subscribe
                    || room_permission == TokenGeneratePermissions::PublishSubscribe,

                ..Default::default()
            },
        };
        let token_response = self.http_client.post("livekit/token", token_request);
        self.http_client
            .map_response::<TokenResponse>(token_response)
    }

    pub fn create_room(&self, name: &str) -> JSONResult<RoomCreationResult> {
        let create_room_request = CreateRoomRequest {
            name: name.to_string(),
            options: RoomOptions::default(),
        };
        let response_result = self
            .http_client
            .post("livekit/create-room", create_room_request);

        self.http_client
            .map_response::<RoomCreationResult>(response_result)
    }

    pub fn list_rooms(&self) -> JSONResult<Vec<RoomCreationResult>> {
        let list_result = self.http_client.get("livekit/list-rooms");

        self.http_client
            .map_response::<Vec<RoomCreationResult>>(list_result)
    }

    pub fn delete_room(self, room_name: &str) -> JSONResult<ApiResponse> {
        let delete_result = self
            .http_client
            .delete(&format!("livekit/delete-room/{}", room_name));
        self.http_client.map_response::<ApiResponse>(delete_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    fn setup_lk_client() -> Option<LiveKitClient> {
        dotenv().ok();
        let app_host: String = env::var("APP_HOST").unwrap_or("localhost".to_string());
        let app_port: String = env::var("APP_PORT").unwrap_or("8081".to_string());
        let base_url: String = format!("http://{}:{}", app_host, app_port);

        let login_token: Option<String> = env::var("LOGIN_TOKEN").ok();

        match login_token {
            Some(token) => {
                let livekit_client = LiveKitClient::new(&base_url, &token);
                Some(livekit_client)
            }
            None => {
                println!("LOGIN_TOKEN not found in environment variables");
                None
            }
        }
    }

    #[test]
    fn test_healthcheck() {
        let livekit_client = setup_lk_client();

        assert!(livekit_client.is_some());
        assert_eq!(livekit_client.unwrap().healthcheck().is_ok(), true);
    }

    #[test]
    fn test_create_delete_room() {
        let livekit_client = setup_lk_client();

        assert!(livekit_client.is_some());
        let livekit_client = livekit_client.unwrap();
        let room_name = "test-room";
        let create_room_result = livekit_client.create_room(room_name);
        assert_eq!(create_room_result.is_ok(), true);
        assert_eq!(create_room_result.unwrap().name, room_name);
        let list_rooms_result = livekit_client.list_rooms();
        assert!(list_rooms_result.is_ok());
        assert!(list_rooms_result.unwrap().len() > 0);

        let delete_room_result = livekit_client.delete_room(room_name);
        assert!(delete_room_result.is_ok());
    }

    #[test]
    fn test_generate_token() {
        let livekit_client = setup_lk_client();

        assert!(livekit_client.is_some());
        let token_result =
            livekit_client
                .unwrap()
                .generate_token("test-user", "test-room", None, None, None);
        assert!(token_result.is_ok());
    }
}
