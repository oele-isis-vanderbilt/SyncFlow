use std::sync::Arc;

use infrastructure::DbPool;
use livekit_protocol::ParticipantInfo;

use crate::{
    livekit::{egress::EgressService, room::RoomService},
    project::session_crud::{self, SessionError},
};
use shared::{
    livekit_models::{TokenRequest, TokenResponse},
    project_models::{LivekitSessionInfo, NewSessionRequest, ProjectSessionResponse},
};

use super::{
    project_crud::{self, Encryptable},
    session_listener::session_listener,
};

pub struct SessionService {
    encryption_key: String,
    pool: Arc<DbPool>,
}

fn get_duration(start_time: i64, end_time: i64) -> i64 {
    end_time - start_time
}

impl SessionService {
    pub fn new(encryption_key: &str, pool: Arc<DbPool>) -> Self {
        SessionService {
            encryption_key: encryption_key.to_string(),
            pool,
        }
    }

    pub async fn create_session(
        &self,
        project_id: &str,
        session: NewSessionRequest,
    ) -> Result<ProjectSessionResponse, SessionError> {
        let new_session = session_crud::create_session(
            project_id,
            &session,
            &self.encryption_key,
            &mut self.pool.get().unwrap(),
        )
        .await?;

        let mut project = project_crud::get_project_by_id(project_id, &mut self.pool.get().unwrap())?;
        project.decrypt(&self.encryption_key)?;

        let session_id = new_session.id;
        let livekit_room_name = new_session.livekit_room_name.clone();
        let pool = self.pool.clone();

        tokio::spawn(async move {
            let mut conn = pool.get().unwrap();
            let _ = session_listener(project, &session_id, &livekit_room_name, &mut conn).await;
        });

        Ok(new_session.into())
    }

    pub fn get_sessions(
        &self,
        project_id: &str,
    ) -> Result<Vec<ProjectSessionResponse>, SessionError> {
        let sessions = session_crud::get_sessions(project_id, &mut self.pool.get().unwrap())?;
        Ok(sessions.into_iter().map(Into::into).collect())
    }

    pub async fn get_participants(
        &self,
        project_id: &str,
        session_id: &str,
    ) -> Result<Vec<ParticipantInfo>, SessionError> {
        let participants = session_crud::get_participants(
            project_id,
            session_id,
            &self.encryption_key,
            &mut self.pool.get().unwrap(),
        )
        .await?;

        Ok(participants)
    }

    pub async fn livekit_session_info(
        &self,
        project_id: &str,
        session_id: &str,
    ) -> Result<LivekitSessionInfo, SessionError> {
        let session =
            session_crud::get_session_if_active(project_id, session_id, &mut self.pool.get().unwrap())?;

        let mut project = project_crud::get_project_by_id(project_id, &mut self.pool.get().unwrap())?;
        project.decrypt(&self.encryption_key)?;

        let room_service: RoomService = (&project).into();
        let egress_service: EgressService = (&project).into();

        let room: Vec<livekit_protocol::Room> = room_service
            .list_rooms(Some(vec![session.livekit_room_name.clone()]))
            .await?;
        let room = room
            .into_iter()
            .next()
            .ok_or(SessionError::InactiveSessionError("Room not found".into()))?;

        let current_timestamp = chrono::Utc::now().timestamp();

        Ok(LivekitSessionInfo {
            room_name: room.name.clone(),
            room_sid: room.sid.clone(),
            participants: room_service.list_participants(&room.name).await?,
            duration: get_duration(room.creation_time, current_timestamp),
            recordings: egress_service.list_egresses(&room.name).await?,
        })
    }

    pub fn get_session(
        &self,
        project_id: &str,
        session_id: &str,
    ) -> Result<ProjectSessionResponse, SessionError> {
        let session = session_crud::get_session(project_id, session_id, &mut self.pool.get().unwrap());
        Ok(session?.into())
    }

    pub fn get_session_token(
        &self,
        project_id: &str,
        session_id: &str,
        token_request: &TokenRequest,
    ) -> Result<TokenResponse, SessionError> {
        session_crud::get_session_token(
            project_id,
            session_id,
            token_request,
            &self.encryption_key,
            &mut self.pool.get().unwrap(),
        )
    }

    pub async fn stop_session(
        &self,
        project_id: &str,
        session_id: &str,
    ) -> Result<ProjectSessionResponse, SessionError> {
        let session = session_crud::stop_session(
            project_id,
            session_id,
            &self.encryption_key,
            &mut self.pool.get().unwrap(),
        )
        .await;

        Ok(session?.into())
    }

    pub async fn delete_session(
        &self,
        project_id: &str,
        session_id: &str,
    ) -> Result<ProjectSessionResponse, SessionError> {
        let session = session_crud::delete_session(
            project_id,
            session_id,
            &self.encryption_key,
            &mut self.pool.get().unwrap(),
        )
        .await;

        Ok(session?.into())
    }
}

impl Clone for SessionService {
    fn clone(&self) -> Self {
        SessionService {
            encryption_key: self.encryption_key.clone(),
            pool: self.pool.clone(),
        }
    }
}
