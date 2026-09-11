use std::str::FromStr;

use domain::models::{
    NewParticipantTrack, NewSessionEgress, NewSessionParticipant, Project, ProjectSessionStatus,
    SessionEgressStatus, SessionEgressType, TrackKind, TrackSource,
};

use diesel::prelude::PgConnection;
use livekit_protocol::{egress_info::Request, EgressInfo, EgressStatus};
use shared::{
    livekit_models::{TokenRequest, VideoGrantsWrapper},
    utils::{get_egress_destination, get_track_id_from_egress},
};
use uuid::Uuid;

use crate::livekit::{
    egress::EgressService,
    room::RoomService,
    room_listener::{listen, RoomTrackKind, RoomTrackSource},
    token::create_token,
};

use super::session_crud::{self, RoomMetadata, SessionError};

impl From<RoomTrackKind> for TrackKind {
    fn from(track_kind: RoomTrackKind) -> Self {
        match track_kind {
            RoomTrackKind::Audio => TrackKind::Audio,
            RoomTrackKind::Video => TrackKind::Video,
            RoomTrackKind::Unknown => TrackKind::Unknown,
        }
    }
}

impl From<RoomTrackSource> for TrackSource {
    fn from(track_source: RoomTrackSource) -> Self {
        match track_source {
            RoomTrackSource::Camera => TrackSource::Camera,
            RoomTrackSource::Microphone => TrackSource::Microphone,
            RoomTrackSource::ScreenShare => TrackSource::ScreenShare,
            RoomTrackSource::ScreenShareAudio => TrackSource::ScreenShareAudio,
            RoomTrackSource::Unknown => TrackSource::Unknown,
        }
    }
}

fn match_session_id_from_metadata(metadata: &str, session_id: &Uuid) -> Result<bool, SessionError> {
    let metadata = RoomMetadata::from_str(metadata)?;
    Ok(metadata.session_id == *session_id)
}

pub async fn session_listener(
    project: Project,
    session_id: &str,
    livekit_room_name: &str,
    conn: &mut PgConnection,
) -> Result<(), SessionError> {
    let session_uuid = Uuid::from_str(session_id).map_err(|_| {
        SessionError::ConfigurationError(format!("Invalid session id: {}", session_id))
    })?;
    let room_service: RoomService = (&project).into();
    let max_retries = 10;
    let mut retries = 0;
    loop {
        let room_name = livekit_room_name.to_string();
        let rooms = room_service.list_rooms(Some(vec![room_name])).await?;
        let room = rooms.iter().find(|room| room.name == livekit_room_name);
        if room.is_none() {
            retries += 1;
            if retries >= max_retries {
                break;
            } else {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
        } else {
            let metadata = room.unwrap().metadata.clone();
            if match_session_id_from_metadata(&metadata, &session_uuid)? {
                break;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    let join_token = create_token(
        &TokenRequest {
            identity: "room_listener".to_string(),
            name: None,
            video_grants: VideoGrantsWrapper {
                room_create: false,
                room_list: false,
                room_record: false,
                room_admin: true,
                room_join: true,
                room: livekit_room_name.to_string(),
                can_publish: false,
                can_subscribe: true,
                hidden: true,
                ..Default::default()
            },
        },
        &project.livekit_server_api_key,
        &project.livekit_server_api_secret,
    )?;

    let room_info = listen(&project.livekit_server_url, &join_token).await?;

    session_crud::update_session_status(session_id, ProjectSessionStatus::Stopped, conn)?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let egress_service: EgressService = (&project).into();
    let egresses = egress_service.list_egresses(livekit_room_name).await?;

    if !egresses.is_empty() {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let egresses = egress_service.list_egresses(livekit_room_name).await?;
            let statuses = egresses
                .iter()
                .map(|egress| egress.status())
                .collect::<Vec<_>>();
            if statuses
                .iter()
                .any(|status| *status == EgressStatus::EgressEnding)
            {
                continue;
            } else {
                break;
            }
        }

        let session_particpants = room_info
            .participants
            .iter()
            .map(|p| NewSessionParticipant {
                session_id: session_uuid,
                identity: p.identity.clone(),
                name: p.name.clone(),
                joined_at: p.joined_at as i64,
                left_at: p.left_at.map(|l| l as i64),
            })
            .collect::<Vec<NewSessionParticipant>>();
        let inserted_participants =
            session_crud::add_session_participants(session_particpants, session_id, conn)?;

        let session_tracks = room_info
            .participants
            .iter()
            .zip(inserted_participants.iter())
            .flat_map(|(participant, inserted_partcipant)| {
                participant.tracks.iter().map(|t| NewParticipantTrack {
                    sid: t.sid.clone(),
                    name: t.name.clone(),
                    participant_id: inserted_partcipant.id,
                    kind: t.track_kind.clone().into(),
                    source: t.track_source.clone().into(),
                })
            })
            .collect::<Vec<NewParticipantTrack>>();

        let inserted_tracks = session_crud::add_participant_tracks(session_tracks, conn)?;

        let egresses = egress_service.list_egresses(livekit_room_name).await?;

        let session_egress_records = egresses
            .iter()
            .map(|egress| {
                let track_id = get_track_id_from_egress(egress);
                let t_p_ids = inserted_tracks
                    .iter()
                    .find(|track| track.sid == track_id)
                    .map(|track| (track.id, track.participant_id));

                let t_id = t_p_ids.map(|(t_id, _)| t_id);
                let p_id = t_p_ids.map(|(_, p_id)| p_id);

                NewSessionEgress {
                    egress_id: egress.egress_id.clone(),
                    track_id,
                    started_at: egress.started_at,
                    destination: get_egress_destination(egress),
                    status: SessionEgressStatus::from_str_name(egress.status().as_str_name())
                        .unwrap_or(SessionEgressStatus::EgressFailed),
                    egress_type: get_egress_type(egress),
                    session_id: session_uuid,
                    room_name: livekit_room_name.to_string(),
                    participant_id: p_id,
                    db_track_id: t_id,
                }
            })
            .collect::<Vec<_>>();

        session_crud::create_session_egresses(session_egress_records, conn)?;
    }
    Ok(())
}

fn get_egress_type(egress: &EgressInfo) -> Option<domain::models::SessionEgressType> {
    if let Some(request) = egress.request.clone() {
        let egress_type = match request {
            Request::RoomComposite(_) => SessionEgressType::RoomComposite,
            Request::Participant(_) => SessionEgressType::Participant,
            Request::Track(_) => SessionEgressType::Track,
            Request::TrackComposite(_) => SessionEgressType::TrackComposite,
            Request::Web(_) => SessionEgressType::Web,
        };
        Some(egress_type)
    } else {
        None
    }
}
