use std::str::FromStr;

use domain::models::{Project, ProjectSessionStatus};

use diesel::prelude::PgConnection;
use uuid::Uuid;

use crate::livekit::room::RoomService;

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
        let rooms = room_service.list_rooms(None).await?;
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

    Ok(())
}
