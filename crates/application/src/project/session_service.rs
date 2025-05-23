use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use domain::{models::ProjectSessionStatus, models::SessionEgressStatus};
use infrastructure::DbPool;
use livekit_protocol::ParticipantInfo;

use crate::{
    livekit::{egress::EgressService, room::RoomService},
    project::session_crud::{self, SessionError},
    rmq::session_notifier::SessionNotifier,
    s3::storage_service::StorageService,
};
use shared::{
    device_models::NewSessionMessage,
    livekit_models::{TokenRequest, TokenResponse},
    project_models::{
        EgressMediaDownloadResponse, EgressResponse, LivekitSessionInfo, MultimediaDetails,
        NewSessionRequest, ProjectSessionResponse, SessionParticipantResponse,
    },
};

use super::{
    devices::device_crud,
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
        session: &NewSessionRequest,
        notifier: &SessionNotifier,
    ) -> Result<ProjectSessionResponse, SessionError> {
        let registered_devices =
            device_crud::list_devices(project_id, &mut self.pool.get().unwrap())
                .unwrap_or_default()
                .into_iter()
                .map(|d| d.device_group)
                .collect::<Vec<String>>();
        let notified_devices = session.device_groups.clone().unwrap_or_default();
        if !notified_devices.is_empty() {
            let unregistered_devices = notified_devices
                .iter()
                .filter(|grp| !registered_devices.contains(grp))
                .cloned()
                .collect::<Vec<String>>();

            if !unregistered_devices.is_empty() {
                return Err(SessionError::InvalidDeviceGroupError(format!(
                    "Invalid device groups: [{}]",
                    unregistered_devices.join(", ")
                )));
            }
        }

        let new_session = session_crud::create_session(
            project_id,
            session,
            &self.encryption_key,
            &mut self.pool.get().unwrap(),
        )
        .await?;

        let mut project =
            project_crud::get_project_by_id(project_id, &mut self.pool.get().unwrap())?;
        project.decrypt(&self.encryption_key)?;

        let session_id = new_session.id;
        let livekit_room_name = new_session.livekit_room_name.clone();
        let pool = self.pool.clone();

        tokio::spawn(async move {
            let mut conn = pool.get().unwrap();
            let _ = session_listener(
                project,
                &session_id.to_string(),
                &livekit_room_name,
                &mut conn,
            )
            .await;
        });

        for grp in notified_devices {
            let routing_key = format!("{}.{}", project_id, grp);
            let new_session_message = NewSessionMessage {
                session_id: session_id.to_string(),
                session_name: new_session.livekit_room_name.clone(),
            };

            let bytes = serde_json::to_vec(&new_session_message).unwrap();

            notifier.publish(&routing_key, bytes).await?;
        }

        Ok(new_session.into())
    }

    pub async fn get_sessions(
        &self,
        project_id: &str,
    ) -> Result<Vec<ProjectSessionResponse>, SessionError> {
        let conn = &mut self.pool.get().unwrap();
        let mut project = project_crud::get_project_by_id(project_id, conn)?;
        let sessions = session_crud::get_sessions(project_id, conn)?;
        let mut session_response: Vec<ProjectSessionResponse> =
            sessions.into_iter().map(Into::into).collect();

        project.decrypt(&self.encryption_key)?;

        let room_service: RoomService = (&project).into();
        let egress_service: EgressService = (&project).into();
        for session_response in session_response.iter_mut() {
            if session_response.status == "Started" {
                let num_participants = room_service
                    .list_participants(&session_response.livekit_room_name)
                    .await?
                    .len() as i64;

                let num_recordings = egress_service
                    .list_egresses(&session_response.livekit_room_name)
                    .await?
                    .len() as i64;

                session_response.num_participants = num_participants;
                session_response.num_recordings = num_recordings;
            } else {
                let (num_participants, num_recordings) =
                    session_crud::get_num_participants_and_egresses(&session_response.id, conn)?;

                session_response.num_participants = num_participants;
                session_response.num_recordings = num_recordings;
            }
        }

        Ok(session_response)
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
        room_name: &str,
    ) -> Result<LivekitSessionInfo, SessionError> {
        let mut project =
            project_crud::get_project_by_id(project_id, &mut self.pool.get().unwrap())?;
        project.decrypt(&self.encryption_key)?;

        let room_service: RoomService = (&project).into();
        let egress_service: EgressService = (&project).into();

        let room: Vec<livekit_protocol::Room> = room_service
            .list_rooms(Some(vec![room_name.to_string()]))
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

    pub async fn get_session(
        &self,
        project_id: &str,
        session_id: &str,
    ) -> Result<ProjectSessionResponse, SessionError> {
        let conn = &mut self.pool.get().unwrap();
        let session = session_crud::get_session(project_id, session_id, conn)?;

        match session.status {
            ProjectSessionStatus::Stopped => {
                let mut project = project_crud::get_project_by_id(project_id, conn)?;
                project.decrypt(&self.encryption_key)?;

                let storage_service: StorageService = (&project).into();

                let (participants, recordings) =
                    session_crud::load_session_participant_tracks_recordings(
                        &session,
                        &mut self.pool.get().unwrap(),
                    )?;

                let mut session_response: ProjectSessionResponse = session.into();

                session_response.participants = participants
                    .into_iter()
                    .map(|(participant, tracks)| {
                        let mut participant_response: SessionParticipantResponse =
                            participant.into();

                        participant_response.tracks = tracks.into_iter().map(Into::into).collect();
                        participant_response
                    })
                    .collect();

                for participant in session_response.participants.iter_mut() {
                    for track in participant.tracks.iter_mut() {
                        if let Some(egress) = recordings
                            .iter()
                            .find(|egress| egress.track_id == track.sid)
                        {
                            if egress.status == SessionEgressStatus::EgressComplete
                                && egress.destination.is_some()
                            {
                                let url = storage_service
                                    .generate_presigned_url(
                                        egress.destination.as_ref().unwrap(),
                                        Some(500),
                                    )
                                    .await?;

                                track.multimedia_details = Some(MultimediaDetails {
                                    file_name: egress
                                        .destination
                                        .as_ref()
                                        .and_then(|d| d.split("/").last().map(|s| s.to_string())),
                                    destination: egress.destination.clone(),
                                    publisher: Some(participant.identity.clone()),
                                    track_id: Some(track.sid.clone()),
                                    presigned_url: Some(url),
                                    presigned_url_expires: Some(500),
                                    recording_start_time: Some(egress.started_at),
                                });
                            }
                        }
                    }
                }
                session_response.recordings = recordings.into_iter().map(Into::into).collect();

                Ok(session_response)
            }
            _ => {
                let mut session_response: ProjectSessionResponse = session.into();

                let lk_session_info = self
                    .livekit_session_info(project_id, &session_response.livekit_room_name)
                    .await?;
                let (participants, egresses) = lk_session_info.into();
                session_response.participants = participants;
                session_response.recordings = egresses;

                Ok(session_response)
            }
        }
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

    pub async fn list_egresses(
        &self,
        _project_id: &str,
        session_id: &str,
    ) -> Result<Vec<EgressResponse>, SessionError> {
        let conn = &mut self.pool.get().unwrap();
        let egresses = session_crud::get_session_egresses(session_id, conn)?;

        Ok(egresses.into_iter().map(Into::into).collect())
    }

    pub async fn get_egress_download_url(
        &self,
        project_id: &str,
        path: &str,
    ) -> Result<EgressMediaDownloadResponse, SessionError> {
        let mut project =
            project_crud::get_project_by_id(project_id, &mut self.pool.get().unwrap())?;
        project.decrypt(&self.encryption_key)?;

        let storage_service: StorageService = (&project).into();

        let url = storage_service
            .generate_presigned_url(path, Some(300))
            .await?;

        Ok(EgressMediaDownloadResponse {
            bucket_name: project.bucket_name.clone(),
            expires_in: SystemTime::now()
                .checked_add(Duration::from_secs(300))
                .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs())
                .unwrap_or(0),
            media_path: path.to_string(),
            media_url: url,
        })
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
