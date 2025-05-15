use std::collections::HashMap;

use livekit_client::{prelude::RemoteParticipant, Room, RoomEvent, RoomOptions};
use thiserror::Error;

pub struct RoomListenerService {
    server_url: String,
    token: String,
}

#[derive(Debug, Error)]
pub enum RoomListenerError {
    #[error("{0}")]
    ConnectionError(String),
}

impl RoomListenerService {
    pub fn new(server_url: &str, token: &str) -> Self {
        Self {
            server_url: server_url.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn listen(
        &mut self,
    ) -> Result<HashMap<String, RemoteParticipant>, RoomListenerError> {
        let mut published_tracks: HashMap<String, RemoteParticipant> = HashMap::new();
        let (_room, mut room_events) =
            Room::connect(&self.server_url, &self.token, RoomOptions::default())
                .await
                .map_err(|err| {
                    RoomListenerError::ConnectionError(format!(
                        "Failed to connect to room: {}",
                        err
                    ))
                })?;
        while let Some(event) = room_events.recv().await {
            match event {
                RoomEvent::TrackSubscribed {
                    track,
                    publication,
                    participant,
                } => {
                    let track_sid = track.sid().to_string();
                    published_tracks.insert(track_sid.clone(), participant.clone());
                }
                RoomEvent::Disconnected { reason } => {
                    log::info!("Disconnected from room: {:?}", reason);
                    break;
                }
                _ => {}
            }
        }
        Ok(published_tracks)
    }
}
