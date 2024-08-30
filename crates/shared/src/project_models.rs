use serde::{Deserialize, Serialize};

use crate::livekit_models::RoomOptions;
use crate::utils::generate_random_session_name;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionRequest {
    pub name: Option<String>,
    pub comments: Option<String>,
    pub empty_timeout: Option<i32>,
    pub max_participants: Option<i32>,
}

impl Default for NewSessionRequest {
    fn default() -> Self {
        NewSessionRequest {
            name: Some(generate_random_session_name()),
            comments: None,
            empty_timeout: Some(600),
            max_participants: Some(100),
        }
    }
}

impl From<NewSessionRequest> for RoomOptions {
    fn from(val: NewSessionRequest) -> Self {
        RoomOptions {
            max_participants: val.max_participants.unwrap_or(100) as u32,
            empty_timeout: val.empty_timeout.unwrap_or(600) as u32,
            metadata: val.comments.unwrap_or("".to_string()),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionResponse {
    pub name: String,
    pub comments: String,
    pub empty_timeout: i32,
    pub max_participants: i32,
    pub livekit_room_name: String,
    pub project_id: String,
    pub status: String,
}