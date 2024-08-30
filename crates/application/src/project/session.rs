use diesel::{prelude::*, PgConnection};
use domain::models::{NewProject, NewProjectSession, ProjectSession};
use domain::schema::syncflow::project_sessions::project_id;
use livekit_api::services::{ServiceError};
use shared::livekit_models::{LivekitRoom, RoomOptions};
use shared::project_models::NewSessionRequest;
use thiserror::Error;
use uuid::Uuid;
use crate::project;
use crate::project::project::Encryptable;

use crate::livekit::room::RoomService;
use crate::users::secret::SecretError;

use super::project::ProjectError;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Database Error: {0}")]
    DatabaseError(# [from] diesel::result::Error),

    #[error("Project Error: {0}")]
    ProjectError(#[from] ProjectError),

    #[error("Livekit Error: {0}")]
    SecretError(#[from] SecretError),

    #[error("Livekit Error: {0}")]
    LiveKitError(#[from] ServiceError),

    #[error("Configuration Error: {0}")]
    ConfigurationError(String),
}

impl From<SessionError> for shared::response_models::Response {
    fn from(val: SessionError) -> Self {
        match val {
            SessionError::DatabaseError(e) => shared::response_models::Response {
                status: 500,
                message: e,
            },
            SessionError::ProjectError(e) => e.into(),
            SessionError::LiveKitError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            SessionError::SecretError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            SessionError::ConfigurationError(e) => shared::response_models::Response {
                status: 500,
                message: e,
            },
        }
    }
}

async fn create_livekit_room(
    api_key: &str,
    api_secret: &str,
    room_name: &str,
    server_url: &str,
    room_opts: &RoomOptions,
) -> Result<LivekitRoom, ServiceError> {
    let room_service = RoomService::new(
        server_url.to_string(),
        api_key.to_string(),
        api_secret.to_string(),
    );
    let room = room_service.create_room(room_name, room_opts.clone()).await?;
    Ok(room.into())
}

pub async fn create_session(
    proj_id: &str,
    session: &NewSessionRequest,
    encryption_key: &str,
    conn: &mut PgConnection
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;


    let mut project = project::project::get_project_by_id(proj_id, conn)?;
    project.decrypt(encryption_key)?;
    let project_uuid = project.id.clone();

   
    let room_opts: RoomOptions = session.clone().into();
    let room_name = session.name.clone().ok_or(
        SessionError::ConfigurationError("Session name is required".to_string()),
    )?;

    let room: LivekitRoom = create_livekit_room(
        &project.livekit_server_api_key,
        &project.livekit_server_api_secret,
        &room_name,
        &project.livekit_server_url,
        &room_opts,
    )
    .await?;

    let new_session = NewProjectSession {
        comments: session.comments.clone(),
        empty_timeout: session.empty_timeout.unwrap_or_default(),
        livekit_room_name: room.name.clone(),
        max_participants: session.max_participants.unwrap_or_default(),
        project_id: project_uuid,
        name: room_name,
    };

    let session = diesel::insert_into(project_sessions)
        .values(&new_session)
        .get_result::<ProjectSession>(conn)?;

    Ok(session)
}


pub fn get_session(
    session_id: &str,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let session_id = Uuid::parse_str(session_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;

    let session = project_sessions
        .filter(id.eq(session_id))
        .first::<ProjectSession>(conn)?;

    Ok(session)
}


pub async fn stop_session(
    proj_id: &str,
    session_id: &str,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let mut project = project::project::get_project_by_id(proj_id, conn)?;

    let session_id = Uuid::parse_str(session_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;

    let liveki
}