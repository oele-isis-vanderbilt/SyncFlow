use livekit_protocol::{EgressInfo, ParticipantInfo};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::livekit_models::RoomOptions;
use crate::utils::generate_random_session_name;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionRequest {
    pub name: Option<String>,
    pub comments: Option<String>,
    pub empty_timeout: Option<i32>,
    pub max_participants: Option<i32>,
    pub auto_recording: Option<bool>,
}

impl NewSessionRequest {
    pub fn get_name(&self) -> String {
        self.name
            .as_ref()
            .take()
            .map(|name| {
                if name.is_empty() {
                    generate_random_session_name()
                } else {
                    name.clone()
                }
            })
            .unwrap_or_else(|| generate_random_session_name())
    }
}

impl Default for NewSessionRequest {
    fn default() -> Self {
        NewSessionRequest {
            name: Some(generate_random_session_name()),
            comments: None,
            empty_timeout: Some(600),
            max_participants: Some(100),
            auto_recording: Some(false),
        }
    }
}

impl From<NewSessionRequest> for RoomOptions {
    fn from(val: NewSessionRequest) -> Self {
        RoomOptions {
            max_participants: val.max_participants.unwrap_or(100) as u32,
            empty_timeout: val.empty_timeout.unwrap_or(600) as u32,
            metadata: val.comments.unwrap_or("".to_string()),
            auto_recording: val.auto_recording.unwrap_or(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSessionResponse {
    pub id: String,
    pub name: String,
    pub started_at: usize,
    pub comments: String,
    pub empty_timeout: i32,
    pub max_participants: i32,
    pub livekit_room_name: String,
    pub project_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsSummary {
    pub num_projects: u32,
    pub num_sessions: u32,
    pub num_active_sessions: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub num_sessions: u32,
    pub num_active_sessions: u32,
    pub num_participants: u32,
    pub num_recordings: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LivekitSessionInfo {
    pub room_name: String,
    pub room_sid: String,
    pub participants: Vec<ParticipantInfo>,
    pub recordings: Vec<EgressInfo>,
    pub duration: i64,
}
