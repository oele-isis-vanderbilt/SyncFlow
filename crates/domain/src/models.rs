use crate::schema::syncflow::{
    api_keys, create_room_actions, delete_room_actions, egress_actions, generate_token_actions,
    list_rooms_actions, login_sessions, users,
};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use shared::user_models::{ApiKeyResponse, ApiKeyResponseWithoutSecret, UserProfile};
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
