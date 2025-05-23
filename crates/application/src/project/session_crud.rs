use diesel::prelude::*;
use std::fmt::Display;
use std::str::FromStr;

use crate::livekit::room::RoomService;
use crate::livekit::room_listener;
use crate::project::project_crud::Encryptable;
use crate::users::secret::SecretError;
use crate::{livekit, project};

use diesel::PgConnection;
use domain::models::{
    NewParticipantTrack, NewProjectSession, NewSessionEgress, NewSessionParticipant,
    ParticipantTrack, ProjectSession, ProjectSessionStatus, SessionEgress, SessionParticipant,
};
use livekit_api::services::ServiceError;
use livekit_client::RoomError;
use livekit_protocol::ParticipantInfo;
use shared::livekit_models::{RoomOptions, TokenRequest, TokenResponse};
use shared::project_models::NewSessionRequest;
use thiserror::Error;
use uuid::Uuid;

use super::project_crud::ProjectError;
use crate::rmq::session_notifier::SessionNotifierError;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Database Error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Project Error: {0}")]
    ProjectError(#[from] ProjectError),

    #[error("Livekit Error: {0}")]
    SecretError(#[from] SecretError),

    #[error("Livekit Error: {0}")]
    LiveKitError(#[from] ServiceError),

    #[error("Livekit Access Token Error: {0}")]
    AccessTokenError(#[from] livekit_api::access_token::AccessTokenError),

    #[error("Livekit Room Error: {0}")]
    RoomError(#[from] RoomError),

    #[error("Configuration Error: {0}")]
    ConfigurationError(String),

    #[error("Duplication Session Name: {0}")]
    DuplicateSessionNameError(String),

    #[error("Inactive Session Error: {0}")]
    InactiveSessionError(String),

    #[error("Session Notifier Error: {0}")]
    SessionNotifierError(#[from] SessionNotifierError),

    #[error("Invalid Device Group Error: {0}")]
    InvalidDeviceGroupError(String),

    #[error("Storage Service Error: {0}")]
    StorageServiceError(#[from] rusoto_credential::CredentialsError),

    #[error("Room Listener Error: {0}")]
    RoomListenerError(#[from] room_listener::RoomListenerError),
}

#[derive(Debug, Clone)]
pub struct RoomMetadata {
    pub session_id: Uuid,
    pub project_id: Uuid,
    pub comments: Option<String>,
}

impl Display for RoomMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "|session_id:{}|project_id:{}|comments:{}|",
            self.session_id,
            self.project_id,
            self.comments.clone().unwrap_or_default()
        )
    }
}

impl FromStr for RoomMetadata {
    type Err = SessionError;

    fn from_str(metadata: &str) -> Result<Self, SessionError> {
        let metadata = metadata.trim_matches('|');
        let metadata: Vec<&str> = metadata.split('|').collect();
        if metadata.len() != 3 {
            Err(SessionError::ConfigurationError(
                "Invalid metadata format".to_string(),
            ))
        } else {
            let session_id = metadata[0].split(":").last().unwrap_or_default();
            let project_id = metadata[1].split(":").last().unwrap_or_default();
            let comments = metadata[2].split(":").last().unwrap_or_default();

            let session_id = Uuid::parse_str(session_id)
                .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;
            let project_id = Uuid::parse_str(project_id)
                .map_err(|_| SessionError::ConfigurationError("Invalid project id".to_string()))?;

            Ok(RoomMetadata {
                session_id,
                project_id,
                comments: Some(comments.to_string()),
            })
        }
    }
}

impl From<SessionError> for shared::response_models::Response {
    fn from(val: SessionError) -> Self {
        match val {
            SessionError::DatabaseError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            SessionError::ProjectError(e) => e.into(),
            SessionError::LiveKitError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            SessionError::AccessTokenError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            SessionError::RoomError(e) => shared::response_models::Response {
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
            SessionError::InactiveSessionError(e) => shared::response_models::Response {
                status: 400,
                message: e,
            },
            SessionError::SessionNotifierError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            SessionError::DuplicateSessionNameError(e) => shared::response_models::Response {
                status: 400,
                message: e.to_string(),
            },
            SessionError::InvalidDeviceGroupError(e) => shared::response_models::Response {
                status: 400,
                message: e,
            },
            SessionError::StorageServiceError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            SessionError::RoomListenerError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
        }
    }
}

pub async fn create_session(
    proj_id: &str,
    session: &NewSessionRequest,
    encryption_key: &str,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let mut project = project::project_crud::get_project_by_id(proj_id, conn)?;
    project.decrypt(encryption_key)?;
    let project_uuid = project.id;

    let room_service: RoomService = (&project).into();
    let room_opts: RoomOptions = session.clone().into();
    let room_name = session.get_name();
    let rooms = room_service
        .list_rooms(Some(vec![room_name.clone()]))
        .await?;
    let duplicate_room_exists = rooms
        .iter()
        .find(|room| room.name == room_name)
        .map(|_| true)
        .unwrap_or(false);

    if duplicate_room_exists {
        return Err(SessionError::DuplicateSessionNameError(format!(
            "Session name {:?} session already exists and is active, please stop the session",
            room_name
        )));
    }

    let room = room_service.create_room(&room_name, room_opts).await?;

    let new_session = NewProjectSession {
        comments: session.comments.clone(),
        empty_timeout: session.empty_timeout.unwrap_or_default(),
        livekit_room_name: room.name.clone(),
        status: ProjectSessionStatus::Started,
        max_participants: session.max_participants.unwrap_or_default(),
        project_id: project_uuid,
        name: room_name,
    };

    let session = diesel::insert_into(project_sessions)
        .values(&new_session)
        .get_result::<ProjectSession>(conn)?;

    let room_metadata = RoomMetadata {
        session_id: session.id,
        project_id: project_uuid,
        comments: session.comments.clone(),
    };

    let _ = room_service
        .update_room_metadata(&session.livekit_room_name, &room_metadata.to_string())
        .await?;

    Ok(session)
}

pub fn get_session(
    proj_id: &str,
    session_id: &str,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let session_id = Uuid::parse_str(session_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid project id".to_string()))?;

    let session = project_sessions
        .filter(id.eq(session_id).and(project_id.eq(proj_uuid)))
        .first::<ProjectSession>(conn)?;

    Ok(session)
}

pub fn get_session_if_active(
    proj_id: &str,
    session_id: &str,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    let session = get_session(proj_id, session_id, conn)?;
    if session.status != ProjectSessionStatus::Started {
        return Err(SessionError::InactiveSessionError(
            "Session is not active".to_string(),
        ));
    }
    Ok(session)
}

pub fn get_session_token(
    proj_id: &str,
    session_id: &str,
    token_request: &TokenRequest,
    encryption_key: &str,
    conn: &mut PgConnection,
) -> Result<TokenResponse, SessionError> {
    let session = get_session_if_active(proj_id, session_id, conn)?;
    let mut project = project::project_crud::get_project_by_id(proj_id, conn)?;
    if project.id != session.project_id {
        Err(SessionError::ConfigurationError(
            "Invalid project id".to_string(),
        ))?
    } else {
        if token_request.video_grants.room != session.livekit_room_name {
            Err(SessionError::ConfigurationError(
                "Invalid room name(mismatch between room and livekit room name)".to_string(),
            ))?
        }
        project.decrypt(encryption_key)?;
        let server_url = project.livekit_server_url.clone();
        let token = livekit::token::create_token(
            token_request,
            &project.livekit_server_api_key,
            &project.livekit_server_api_secret,
        )?;
        Ok(TokenResponse::new(
            token,
            token_request.identity.clone(),
            Some(server_url),
        ))
    }
}

pub async fn delete_session(
    proj_id: &str,
    session_id: &str,
    encryption_key: &str,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let session = get_session(proj_id, session_id, conn)?;
    let session = diesel::delete(project_sessions.filter(id.eq(session.id)))
        .get_result::<ProjectSession>(conn)?;

    if session.status != ProjectSessionStatus::Stopped {
        let mut project = project::project_crud::get_project_by_id(proj_id, conn)?;
        project.decrypt(encryption_key)?;

        let room_service: RoomService = (&project).into();
        room_service.delete_room(&session.livekit_room_name).await?;

        let session_status = ProjectSessionStatus::Stopped;
        let _ = update_session_status(session_id, session_status, conn);
    }

    Ok(session)
}

pub fn get_sessions(
    proj_id: &str,
    conn: &mut PgConnection,
) -> Result<Vec<ProjectSession>, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid project id".to_string()))?;

    let sessions = project_sessions
        .filter(project_id.eq(proj_uuid))
        .load::<ProjectSession>(conn)?;

    Ok(sessions)
}

pub async fn get_participants(
    proj_id: &str,
    session_id: &str,
    encryption_key: &str,
    conn: &mut PgConnection,
) -> Result<Vec<ParticipantInfo>, SessionError> {
    let project_session = get_session_if_active(proj_id, session_id, conn)?;

    let mut project = project::project_crud::get_project_by_id(proj_id, conn)?;
    if project.id != project_session.project_id {
        return Err(SessionError::ConfigurationError(
            "Invalid project id".to_string(),
        ));
    }

    project.decrypt(encryption_key)?;

    let room_service: RoomService = (&project).into();

    let participants = room_service
        .list_participants(&project_session.livekit_room_name)
        .await?;

    Ok(participants)
}

pub async fn stop_session(
    proj_id: &str,
    session_id: &str,
    encryption_key: &str,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let mut project = project::project_crud::get_project_by_id(proj_id, conn)?;
    project.decrypt(encryption_key)?;
    let room_service: RoomService = (&project).into();

    let session = project::session_crud::get_session(proj_id, session_id, conn)?;

    if session.status != ProjectSessionStatus::Started {
        return Err(SessionError::InactiveSessionError(
            "Session is not active".to_string(),
        ));
    }

    room_service.delete_room(&session.livekit_room_name).await?;

    let session = diesel::update(project_sessions.filter(id.eq(session.id)))
        .set(status.eq(ProjectSessionStatus::Stopped))
        .get_result::<ProjectSession>(conn)?;

    Ok(session)
}

pub fn update_session_status(
    session_id: &str,
    session_status: ProjectSessionStatus,
    conn: &mut PgConnection,
) -> Result<ProjectSession, SessionError> {
    use domain::schema::syncflow::project_sessions::dsl::*;
    let session_uuid = Uuid::parse_str(session_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;
    let session = diesel::update(project_sessions.filter(id.eq(session_uuid)))
        .set(status.eq(session_status))
        .get_result::<ProjectSession>(conn);

    let session = session?;
    Ok(session)
}

pub fn create_session_egresses(
    egresses: Vec<NewSessionEgress>,
    conn: &mut PgConnection,
) -> Result<Vec<SessionEgress>, SessionError> {
    use domain::schema::syncflow::session_egresses::dsl::*;

    let egresses = diesel::insert_into(session_egresses)
        .values(&egresses)
        .get_results::<SessionEgress>(conn)?;

    Ok(egresses)
}

pub fn get_session_egresses(
    sess_id: &str,
    conn: &mut PgConnection,
) -> Result<Vec<SessionEgress>, SessionError> {
    use domain::schema::syncflow::session_egresses::dsl::*;
    let sess_uuid = Uuid::parse_str(sess_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;

    let egresses = session_egresses
        .filter(session_id.eq(sess_uuid))
        .load::<SessionEgress>(conn)
        .unwrap();

    Ok(egresses)
}

pub fn add_session_participants(
    participants: Vec<NewSessionParticipant>,
    sess_id: &str,
    conn: &mut PgConnection,
) -> Result<Vec<SessionParticipant>, SessionError> {
    use domain::schema::syncflow::session_participants::dsl::*;

    let _sess_uuid = Uuid::parse_str(sess_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;

    let participants = diesel::insert_into(session_participants)
        .values(&participants)
        .get_results::<SessionParticipant>(conn)?;

    Ok(participants)
}

#[allow(clippy::type_complexity)]
pub fn load_session_participant_tracks_recordings(
    session: &ProjectSession,
    conn: &mut PgConnection,
) -> Result<
    (
        Vec<(SessionParticipant, Vec<ParticipantTrack>)>,
        Vec<SessionEgress>,
    ),
    SessionError,
> {
    let participants =
        SessionParticipant::belonging_to(session).load::<SessionParticipant>(conn)?;

    let tracks = ParticipantTrack::belonging_to(&participants)
        .load::<ParticipantTrack>(conn)?
        .grouped_by(&participants);

    let egresses = SessionEgress::belonging_to(session).load::<SessionEgress>(conn)?;

    let particpant_and_tracks = participants.into_iter().zip(tracks).collect::<Vec<_>>();

    Ok((particpant_and_tracks, egresses))
}

pub fn add_participant_tracks(
    tracks: Vec<NewParticipantTrack>,
    conn: &mut PgConnection,
) -> Result<Vec<ParticipantTrack>, SessionError> {
    use domain::schema::syncflow::participant_tracks::dsl::*;
    let tracks = diesel::insert_into(participant_tracks)
        .values(&tracks)
        .get_results::<ParticipantTrack>(conn)?;

    Ok(tracks)
}

pub fn get_num_participants_and_egresses(
    sess_id: &str,
    conn: &mut PgConnection,
) -> Result<(i64, i64), SessionError> {
    use domain::schema::syncflow::session_egresses::dsl as session_egresses_dsl;
    use domain::schema::syncflow::session_participants::dsl as session_participants_dsl;

    let sess_uuid = Uuid::parse_str(sess_id)
        .map_err(|_| SessionError::ConfigurationError("Invalid session id".to_string()))?;

    let num_participants = session_participants_dsl::session_participants
        .filter(session_participants_dsl::session_id.eq(sess_uuid))
        .count()
        .get_result::<i64>(conn)?;

    let num_recordings = session_egresses_dsl::session_egresses
        .filter(session_egresses_dsl::session_id.eq(sess_uuid))
        .count()
        .get_result::<i64>(conn)?;

    Ok((num_participants, num_recordings))
}
