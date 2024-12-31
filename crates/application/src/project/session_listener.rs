use std::str::FromStr;

use domain::models::{
    NewSessionEgress, Project, ProjectSessionStatus, SessionEgressStatus, SessionEgressType,
};

use diesel::prelude::PgConnection;
use livekit_protocol::{egress_info::Request, EgressInfo, EgressStatus};
use uuid::Uuid;

use crate::livekit::{egress::EgressService, room::RoomService};

use super::session_crud::{self, RoomMetadata, SessionError};

fn match_session_id_from_metadata(metadata: &str, session_id: &Uuid) -> Result<bool, SessionError> {
    let metadata = RoomMetadata::from_str(metadata)?;
    Ok(metadata.session_id == *session_id)
}

pub async fn session_listener(
    project: Project,
    session_id: &Uuid,
    livekit_room_name: &str,
    conn: &mut PgConnection,
) -> Result<(), SessionError> {
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
            if !match_session_id_from_metadata(&metadata, session_id)? {
                break;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

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
        let egresses = egress_service.list_egresses(livekit_room_name).await?;

        let session_egress_records = egresses
            .iter()
            .map(|egress| NewSessionEgress {
                egress_id: egress.egress_id.clone(),
                track_id: get_track_id_from_egress(egress),
                started_at: egress.started_at,
                destination: get_egress_destination(egress),
                status: SessionEgressStatus::from_str_name(egress.status().as_str_name())
                    .unwrap_or(SessionEgressStatus::EgressFailed),
                egress_type: get_egress_type(egress),
                session_id: *session_id,
                room_name: livekit_room_name.to_string(),
            })
            .collect::<Vec<_>>();

        println!("Creating session egresses: {:#?}", session_egress_records);

        session_crud::create_session_egresses(session_egress_records, conn)?;
    }
    Ok(())
}

fn get_track_id_from_egress(egress: &EgressInfo) -> String {
    if let Some(request) = egress.request.clone() {
        match request {
            Request::RoomComposite(_) => "RoomComposite".to_string(),
            Request::Participant(_) => "Participant".to_string(),
            Request::Track(req) => req.track_id.clone(),
            Request::TrackComposite(req) => {
                format!(
                    "TrackComposite-{}-{}",
                    req.audio_track_id, req.video_track_id
                )
            }
            Request::Web(_) => "Web".to_string(),
        }
    } else {
        "Unknown".to_string()
    }
}

fn get_egress_destination(egress: &EgressInfo) -> Option<String> {
    if egress.status() == EgressStatus::EgressComplete {
        let all_destinations = egress
            .file_results
            .iter()
            .map(|dest| dest.filename.clone())
            .collect::<Vec<String>>();

        Some(all_destinations.join(","))
    } else {
        None
    }
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
