use livekit_api::access_token::VideoGrants;
use livekit_protocol::Room;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenRequest {
    pub identity: String,
    #[serde(default)]
    pub video_grants: VideoGrantsWrapper,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct VideoGrantsWrapper {
    // actions on rooms
    pub room_create: bool,
    pub room_list: bool,
    pub room_record: bool,

    // actions on a particular room
    pub room_admin: bool,
    pub room_join: bool,
    pub room: String,

    // permissions within a room
    pub can_publish: bool,
    pub can_subscribe: bool,
    pub can_publish_data: bool,

    // TrackSource types that a participant may publish.
    // When set, it supercedes CanPublish. Only sources explicitly set here can be published
    pub can_publish_sources: Vec<String>, // keys keep track of each source

    // by default, a participant is not allowed to update its own metadata
    pub can_update_own_metadata: bool,

    // actions on ingresses
    pub ingress_admin: bool, // applies to all ingress

    // participant is not visible to other participants (useful when making bots)
    pub hidden: bool,

    // indicates to the room that current participant is a recorder
    pub recorder: bool,
}

impl From<VideoGrants> for VideoGrantsWrapper {
    fn from(value: VideoGrants) -> Self {
        VideoGrantsWrapper {
            room_create: value.room_create,
            room_list: value.room_list,
            room_record: value.room_record,
            room_admin: value.room_admin,
            room_join: value.room_join,
            room: value.room,
            can_publish: value.can_publish,
            can_subscribe: value.can_subscribe,
            can_publish_data: value.can_publish_data,
            can_publish_sources: value.can_publish_sources,
            can_update_own_metadata: value.can_update_own_metadata,
            ingress_admin: value.ingress_admin,
            hidden: value.hidden,
            recorder: value.recorder,
        }
    }
}

impl Into<VideoGrants> for VideoGrantsWrapper {
    fn into(self) -> VideoGrants {
        VideoGrants {
            room_create: self.room_create,
            room_list: self.room_list,
            room_record: self.room_record,
            room_admin: self.room_admin,
            room_join: self.room_join,
            room: self.room,
            can_publish: self.can_publish,
            can_subscribe: self.can_subscribe,
            can_publish_data: self.can_publish_data,
            can_publish_sources: self.can_publish_sources,
            can_update_own_metadata: self.can_update_own_metadata,
            ingress_admin: self.ingress_admin,
            hidden: self.hidden,
            recorder: self.recorder,
        }
    }
}

impl Default for VideoGrantsWrapper {
    fn default() -> Self {
        VideoGrants::default().into()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    token: String,
    identity: String,
}

impl TokenResponse {
    pub fn new(token: String, identity: String) -> Self {
        TokenResponse { token, identity }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoomCreationResult {
    pub sid: String,
    pub name: String,
    pub empty_timeout: u32,
    pub max_participants: u32,
    pub creation_time: i64,
    pub turn_password: String,
    pub enabled_codecs: Vec<String>,
    pub metadata: String,
    pub num_participants: u32,
    pub num_publishers: u32,
    pub active_recording: bool,
}

impl From<Room> for RoomCreationResult {
    fn from(value: Room) -> Self {
        RoomCreationResult {
            sid: value.sid,
            name: value.name,
            empty_timeout: value.empty_timeout,
            max_participants: value.max_participants,
            creation_time: value.creation_time,
            turn_password: value.turn_password,
            enabled_codecs: value
                .enabled_codecs
                .iter()
                .map(|c| c.mime.clone())
                .collect(),
            metadata: value.metadata,
            num_participants: value.num_participants,
            num_publishers: value.num_publishers,
            active_recording: value.active_recording,
        }
    }
}
