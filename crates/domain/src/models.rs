use crate::schema::syncflow::{
    api_keys, create_room_actions, delete_room_actions, egress_actions, generate_token_actions,
    list_rooms_actions, login_sessions, project_api_keys, project_devices, project_sessions,
    projects, users,
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

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable, Queryable)]
#[diesel(table_name = create_room_actions)]
pub struct CreateRoomAction {
    pub id: i32,
    pub room_name: String,
    pub user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = create_room_actions)]
pub struct NewCreateRoomAction {
    pub room_name: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable, Queryable)]
#[diesel(table_name = delete_room_actions)]
pub struct DeleteRoomAction {
    pub id: i32,
    pub room_name: String,
    pub user_id: i32,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = delete_room_actions)]
pub struct NewDeleteRoomAction {
    pub room_name: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable, Queryable)]
#[diesel(table_name = list_rooms_actions)]
pub struct ListRoomsAction {
    pub id: i32,
    pub user_id: i32,
    pub listed_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = list_rooms_actions)]
pub struct NewListRoomsAction {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable, Queryable)]
#[diesel(table_name = generate_token_actions)]
pub struct GenerateTokenAction {
    pub id: i32,
    pub user_id: i32,
    pub token_identity: String,
    pub token_room: String,
    pub generated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = generate_token_actions)]
pub struct NewGenerateTokenAction {
    pub user_id: i32,
    pub token_identity: String,
    pub token_room: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, DbEnum)]
#[ExistingTypePath = "crate::schema::syncflow::sql_types::EgressDestination"]
#[DbValueStyle = "PascalCase"]
pub enum EgressDestination {
    S3,
    LocalFile,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, DbEnum)]
#[ExistingTypePath = "crate::schema::syncflow::sql_types::EgressType"]
#[DbValueStyle = "PascalCase"]
pub enum EgressType {
    RoomComposite,
    TrackComposite,
    Participant,
    Track,
    Web,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = egress_actions)]
pub struct UserEgressAction {
    pub id: i32,
    pub user_id: i32,
    pub egress_id: String,
    pub room_name: String,

    pub egress_destination_path: String,
    pub egress_destination_root: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub success: bool,
    pub egress_destination: EgressDestination,
    pub egress_type: EgressType,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = egress_actions)]
pub struct NewUserEgressAction {
    pub user_id: i32,
    pub egress_id: String,
    pub room_name: String,
    pub egress_destination_path: String,
    pub egress_destination_root: String,
    pub success: bool,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub egress_destination: EgressDestination,
    pub egress_type: EgressType,
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
