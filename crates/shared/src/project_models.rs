use livekit_protocol::egress_info::Request;
use livekit_protocol::{participant_info, EgressInfo, ParticipantInfo};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::livekit_models::RoomOptions;
use crate::utils::{
    generate_random_session_name, get_egress_destination, get_track_id_from_egress,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionRequest {
    pub name: Option<String>,
    pub comments: Option<String>,
    pub empty_timeout: Option<i32>,
    pub max_participants: Option<i32>,
    pub auto_recording: Option<bool>,
    pub device_groups: Option<Vec<String>>,
}

impl NewSessionRequest {
    pub fn get_name(&self) -> String {
        self.name
            .as_ref()
            .map(|name| {
                if name.is_empty() {
                    generate_random_session_name()
                } else {
                    name.clone()
                }
            })
            .unwrap_or_else(generate_random_session_name)
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
            device_groups: None,
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
    pub num_participants: i64,
    pub num_recordings: i64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub participants: Vec<SessionParticipantResponse>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recordings: Vec<EgressResponse>,
    pub duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionParticipantResponse {
    pub id: String,
    pub identity: String,
    pub name: String,
    pub joined_at: i64,
    pub left_at: Option<i64>,
    pub session_id: String,
    pub tracks: Vec<ParticipantTrackResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTrackResponse {
    pub id: String,
    pub sid: String,
    pub name: Option<String>,
    pub kind: String,
    pub source: String,
    pub participant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multimedia_details: Option<MultimediaDetails>,
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

impl From<LivekitSessionInfo> for (Vec<SessionParticipantResponse>, Vec<EgressResponse>) {
    fn from(value: LivekitSessionInfo) -> Self {
        let participants = value
            .participants
            .into_iter()
            .filter_map(|participant| match participant.kind() {
                participant_info::Kind::Standard => Some(SessionParticipantResponse {
                    id: participant.sid.clone(),
                    identity: participant.identity.clone(),
                    name: participant.name.clone(),
                    joined_at: participant.joined_at * 1E9 as i64,
                    left_at: None,
                    session_id: value.room_sid.clone(),
                    tracks: participant
                        .tracks
                        .into_iter()
                        .map(|track| ParticipantTrackResponse {
                            id: track.sid.clone(),
                            sid: track.sid.clone(),
                            name: Some(track.name.clone()),
                            kind: track.r#type().as_str_name().to_string(),
                            source: track.source().as_str_name().to_string(),
                            participant_id: participant.identity.clone(),
                            multimedia_details: None,
                        })
                        .collect(),
                }),
                _ => None,
            })
            .collect();

        let recordings = value
            .recordings
            .into_iter()
            .map(|egress| EgressResponse {
                id: egress.egress_id.clone(),
                track_id: get_track_id_from_egress(&egress),
                egress_id: egress.egress_id.clone(),
                started_at: egress.started_at,
                status: egress.status().as_str_name().to_string(),
                db_track_id: None,
                egress_type: match &egress.request {
                    Some(request) => {
                        let etype = match request {
                            Request::RoomComposite(_) => "RoomComposite".to_string(),
                            Request::Participant(_) => "Participant".to_string(),
                            Request::Track(_) => "Track".to_string(),
                            Request::TrackComposite(_) => "TrackComposite".to_string(),
                            Request::Web(_) => "Web".to_string(),
                        };
                        Some(etype)
                    }
                    _ => None,
                },
                destination: get_egress_destination(&egress),
                room_name: value.room_name.clone(),
                participant_id: None,
                session_id: value.room_sid.clone(),
            })
            .collect();

        (participants, recordings)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EgressMediaPath {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EgressMediaDownloadResponse {
    pub media_path: String,
    pub media_url: String,
    pub bucket_name: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EgressResponse {
    pub id: String,
    pub track_id: String,
    pub egress_id: String,
    pub started_at: i64,
    pub egress_type: Option<String>,
    pub status: String,
    pub destination: Option<String>,
    pub room_name: String,
    pub session_id: String,
    pub participant_id: Option<String>,
    pub db_track_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MultimediaDetails {
    pub file_name: Option<String>,
    pub destination: Option<String>,
    pub publisher: Option<String>,
    pub track_id: Option<String>,
    pub presigned_url: Option<String>,
    pub presigned_url_expires: Option<i64>,
    pub recording_start_time: Option<i64>,
}
