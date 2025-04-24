use crate::schema::syncflow::{
    api_keys, login_sessions, project_api_keys, project_devices, project_sessions, projects,
    session_egresses, users,
};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use shared::{
    device_models::DeviceResponse,
    project_models::ProjectSessionResponse,
    user_models::{ApiKeyResponse, ApiKeyResponseWithoutSecret, ProjectInfo, UserProfile},
};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,

    #[diesel(column_name = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,

    #[diesel(column_name = "updatedAt")]
    pub updated_at: Option<chrono::NaiveDateTime>,

    pub oauth_provider: Option<String>,
    pub oauth_provider_user_id: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub organization: Option<String>,
    pub job_role: Option<String>,
}

impl From<User> for UserProfile {
    fn from(value: User) -> Self {
        UserProfile {
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            organization: value.organization,
            job_role: value.job_role,
            middle_name: value.middle_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Default)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub oauth_provider: Option<String>,
    pub oauth_provider_user_id: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub organization: Option<String>,
    pub job_role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = login_sessions)]
pub struct LoginSession {
    pub session_id: Uuid,
    pub user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = login_sessions)]
pub struct NewLoginSession {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, DbEnum, Eq, PartialEq)]
#[ExistingTypePath = "crate::schema::syncflow::sql_types::KeyType"]
#[DbValueStyle = "PascalCase"]
pub enum KeyType {
    Login,
    Api,
}

#[derive(Queryable, Serialize, Deserialize, Debug, ToSchema)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: i32,
    pub key: String,
    pub user_id: i32,
    pub secret: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub valid: bool,
    pub comment: Option<String>,
    pub key_type: KeyType,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(value: ApiKey) -> Self {
        ApiKeyResponse {
            key: value.key,
            secret: value.secret,
            comment: value.comment.unwrap_or_default(),
            created_at: value
                .created_at
                .map(|c| c.and_utc().timestamp() as usize)
                .unwrap_or_default(),
            project_id: None,
        }
    }
}

impl From<ApiKey> for ApiKeyResponseWithoutSecret {
    fn from(value: ApiKey) -> Self {
        ApiKeyResponseWithoutSecret {
            id: value.id,
            key: value.key,
            comment: value.comment.unwrap_or_default(),
            created_at: value
                .created_at
                .map(|c| c.and_utc().timestamp() as usize)
                .unwrap_or_default(),
            project_id: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey {
    pub key: String,
    pub key_type: KeyType,
    pub user_id: i32,
    pub secret: String,
    pub valid: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone)]
#[ExistingTypePath = "crate::schema::syncflow::sql_types::StorageType"]
#[DbValueStyle = "PascalCase"]
pub enum StorageType {
    S3,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone, ToSchema)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: Uuid,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub livekit_server_url: String,
    pub livekit_server_api_key: String,
    pub livekit_server_api_secret: String,
    pub storage_type: StorageType,
    pub bucket_name: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Project {
    pub fn get_recording_root(&self) -> String {
        format!("{}-{}", self.name, self.id,)
    }
}

impl From<Project> for ProjectInfo {
    fn from(value: Project) -> Self {
        ProjectInfo {
            id: value.id.to_string(),
            name: value.name,
            livekit_server_url: value.livekit_server_url,
            bucket_name: value.bucket_name,
            description: value.description,
            endpoint: value.endpoint,
            storage_type: match value.storage_type {
                StorageType::S3 => "s3".to_string(),
            },
            last_updated: value
                .updated_at
                .map(|c| c.and_utc().timestamp() as usize)
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub livekit_server_url: String,
    pub livekit_server_api_key: String,
    pub livekit_server_api_secret: String,
    pub storage_type: StorageType,
    pub bucket_name: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
}

impl From<Project> for NewProject {
    fn from(value: Project) -> Self {
        NewProject {
            user_id: value.user_id,
            name: value.name,
            description: value.description,
            livekit_server_url: value.livekit_server_url,
            livekit_server_api_key: value.livekit_server_api_key,
            livekit_server_api_secret: value.livekit_server_api_secret,
            storage_type: value.storage_type,
            bucket_name: value.bucket_name,
            endpoint: value.endpoint,
            access_key: value.access_key,
            secret_key: value.secret_key,
            region: value.region,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, Eq, PartialEq)]
#[ExistingTypePath = "crate::schema::syncflow::sql_types::ProjectSessionStatus"]
#[DbValueStyle = "PascalCase"]
pub enum ProjectSessionStatus {
    Created,
    Started,
    Stopped,
}

impl ProjectSessionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectSessionStatus::Created => "Created",
            ProjectSessionStatus::Started => "Started",
            ProjectSessionStatus::Stopped => "Stopped",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = project_sessions)]
pub struct ProjectSession {
    pub id: Uuid,
    pub name: String,
    pub comments: Option<String>,
    pub empty_timeout: i32,
    pub max_participants: i32,
    pub livekit_room_name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub status: ProjectSessionStatus,
    pub project_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = project_sessions)]
pub struct NewProjectSession {
    pub name: String,
    pub comments: Option<String>,
    pub empty_timeout: i32,
    pub max_participants: i32,
    pub livekit_room_name: String,
    pub status: ProjectSessionStatus,
    pub project_id: Uuid,
}

impl From<ProjectSession> for ProjectSessionResponse {
    fn from(value: ProjectSession) -> Self {
        ProjectSessionResponse {
            id: value.id.to_string(),
            name: value.name,
            started_at: value
                .created_at
                .map(|c| c.and_utc().timestamp() as usize)
                .unwrap_or_default(),
            status: value.status.as_str().to_string(),
            comments: value.comments.unwrap_or_default(),
            empty_timeout: value.empty_timeout,
            max_participants: value.max_participants,
            livekit_room_name: value.livekit_room_name,
            project_id: value.project_id.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable)]
#[diesel(table_name = project_api_keys)]
pub struct ProjectAPIKey {
    pub id: i32,
    pub api_key: String,
    pub api_secret: String,
    pub comments: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: i32,
    pub project_id: Uuid,
}

impl From<ProjectAPIKey> for ApiKeyResponse {
    fn from(value: ProjectAPIKey) -> Self {
        ApiKeyResponse {
            key: value.api_key,
            secret: value.api_secret,
            comment: value.comments.unwrap_or_default(),
            created_at: value.created_at.and_utc().timestamp() as usize,
            project_id: Some(value.project_id.to_string()),
        }
    }
}

impl From<ProjectAPIKey> for ApiKeyResponseWithoutSecret {
    fn from(value: ProjectAPIKey) -> Self {
        ApiKeyResponseWithoutSecret {
            id: value.id,
            key: value.api_key,
            comment: value.comments.unwrap_or_default(),
            created_at: value.created_at.and_utc().timestamp() as usize,
            project_id: Some(value.project_id.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable)]
#[diesel(table_name = project_api_keys)]
pub struct NewProjectAPIKey {
    pub api_key: String,
    pub api_secret: String,
    pub comments: Option<String>,
    pub user_id: i32,
    pub project_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = project_devices)]
pub struct ProjectDevice {
    pub id: Uuid,
    pub device_name: String,
    pub device_group: String,
    pub comments: Option<String>,
    pub registered_at: chrono::NaiveDateTime,
    pub project_id: Uuid,
    pub registered_by: i32,
}

impl ProjectDevice {
    pub fn into_device_response(&self, routing_key: &str, exchange_name: &str) -> DeviceResponse {
        DeviceResponse {
            id: self.id.to_string(),
            group: self.device_group.clone(),
            comments: self.comments.clone(),
            name: self.device_name.clone(),
            registered_at: self.registered_at.and_utc().timestamp() as usize,
            registered_by: self.registered_by,
            project_id: self.project_id.to_string(),
            session_notification_exchange_name: Some(exchange_name.to_string()),
            session_notification_binding_key: Some(routing_key.to_string()),
        }
    }
}

impl From<ProjectDevice> for DeviceResponse {
    fn from(value: ProjectDevice) -> Self {
        DeviceResponse {
            id: value.id.to_string(),
            group: value.device_group,
            comments: value.comments,
            name: value.device_name,
            registered_at: value.registered_at.and_utc().timestamp() as usize,
            registered_by: value.registered_by,
            project_id: value.project_id.to_string(),
            session_notification_exchange_name: None,
            session_notification_binding_key: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = project_devices)]
pub struct NewProjectDevice {
    pub device_name: String,
    pub device_group: String,
    pub comments: Option<String>,
    pub project_id: Uuid,
    pub registered_by: i32,
}

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, Eq, PartialEq)]
#[ExistingTypePath = "crate::schema::syncflow::sql_types::SessionEgressStatus"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum SessionEgressStatus {
    #[serde(rename = "EGRESS_STARTING")]
    EgressStarting,
    #[serde(rename = "EGRESS_ACTIVE")]
    EgressActive,
    #[serde(rename = "EGRESS_ENDING")]
    EgressEnding,
    #[serde(rename = "EGRESS_COMPLETE")]
    EgressComplete,
    #[serde(rename = "EGRESS_FAILED")]
    EgressAborted,
    #[serde(rename = "EGRESS_ABORTED")]
    EgressFailed,
    #[serde(rename = "EGRESS_LIMIT_REACHED")]
    EgressLimitReached,
}

impl SessionEgressStatus {
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "EGRESS_STARTING" => Some(Self::EgressStarting),
            "EGRESS_ACTIVE" => Some(Self::EgressActive),
            "EGRESS_ENDING" => Some(Self::EgressEnding),
            "EGRESS_COMPLETE" => Some(Self::EgressComplete),
            "EGRESS_FAILED" => Some(Self::EgressFailed),
            "EGRESS_ABORTED" => Some(Self::EgressAborted),
            "EGRESS_LIMIT_REACHED" => Some(Self::EgressLimitReached),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SessionEgressStatus::EgressStarting => "EGRESS_STARTING",
            SessionEgressStatus::EgressActive => "EGRESS_ACTIVE",
            SessionEgressStatus::EgressEnding => "EGRESS_ENDING",
            SessionEgressStatus::EgressComplete => "EGRESS_COMPLETE",
            SessionEgressStatus::EgressFailed => "EGRESS_FAILED",
            SessionEgressStatus::EgressAborted => "EGRESS_ABORTED",
            SessionEgressStatus::EgressLimitReached => "EGRESS_LIMIT_REACHED",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, Eq, PartialEq)]
#[ExistingTypePath = "crate::schema::syncflow::sql_types::SessionEgressType"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum SessionEgressType {
    #[serde(rename = "ROOM_COMPOSITE")]
    RoomComposite,
    #[serde(rename = "PARTICIPANT")]
    Participant,
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "TRACK_COMPOSITE")]
    TrackComposite,
    #[serde(rename = "TRACK")]
    Track,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = session_egresses)]
#[serde(rename_all = "camelCase")]
pub struct SessionEgress {
    pub id: Uuid,
    pub track_id: String,
    pub egress_id: String,
    pub started_at: i64,
    pub egress_type: Option<SessionEgressType>,
    pub status: SessionEgressStatus,
    pub destination: Option<String>,
    pub room_name: String,
    pub session_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = session_egresses)]
pub struct NewSessionEgress {
    pub track_id: String,
    pub egress_id: String,
    pub started_at: i64,
    pub egress_type: Option<SessionEgressType>,
    pub status: SessionEgressStatus,
    pub destination: Option<String>,
    pub room_name: String,
    pub session_id: Uuid,
}
