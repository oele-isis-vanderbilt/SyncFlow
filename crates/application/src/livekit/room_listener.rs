use livekit_client::{
    participant::ParticipantKind,
    prelude::RemoteParticipant,
    track::{self, RemoteTrack},
    Room, RoomEvent, RoomOptions,
};
use thiserror::Error;

#[derive(Clone, Debug)]
pub enum RoomTrackKind {
    Audio,
    Video,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct RoomParticipant {
    pub identity: String,
    pub name: String,
    pub joined_at: u64,
    pub left_at: Option<u64>,
    pub tracks: Vec<RoomTrack>,
}

impl From<RemoteParticipant> for RoomParticipant {
    fn from(participant: RemoteParticipant) -> Self {
        Self {
            identity: participant.identity().to_string(),
            name: participant.name().to_string(),
            joined_at: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            left_at: None,
            tracks: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RoomTrackSource {
    Camera,
    Microphone,
    ScreenShare,
    ScreenShareAudio,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct RoomTrack {
    pub sid: String,
    pub name: Option<String>,
    pub track_kind: RoomTrackKind,
    pub track_source: RoomTrackSource,
}

#[derive(Clone, Debug)]
pub struct RoomInfo {
    pub room_name: String,
    pub participants: Vec<RoomParticipant>,
}

impl From<RoomInfoCapturer> for RoomInfo {
    fn from(capturer: RoomInfoCapturer) -> Self {
        capturer.room_info
    }
}

#[derive(Debug)]
struct RoomInfoCapturer {
    room_info: RoomInfo,
}

impl RoomInfoCapturer {
    pub fn new(room_name: &str) -> Self {
        Self {
            room_info: RoomInfo {
                room_name: room_name.to_string(),
                participants: Vec::new(),
            },
        }
    }

    pub fn mark_participant_joined(&mut self, participant: RemoteParticipant) {
        if participant.kind() == ParticipantKind::Standard {
            // To Do: Account for rejoin/quit times...
            self.room_info.participants.push(participant.into());
        }
    }

    pub fn mark_participant_left(&mut self, participant: RemoteParticipant) {
        if let Some(p) = self
            .room_info
            .participants
            .iter_mut()
            .find(|p| p.identity == participant.identity().to_string())
        {
            p.left_at = Some(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64);
        }
    }

    pub fn mark_track_published(&mut self, track: RemoteTrack, participant: RemoteParticipant) {
        let track_kind = match track.kind() {
            livekit_client::track::TrackKind::Audio => RoomTrackKind::Audio,
            livekit_client::track::TrackKind::Video => RoomTrackKind::Video,
        };
        let track_sid = track.sid().to_string();
        let track_name = track.name();
        let track_source = match track.source() {
            track::TrackSource::Camera => RoomTrackSource::Camera,
            track::TrackSource::Microphone => RoomTrackSource::Microphone,
            track::TrackSource::Screenshare => RoomTrackSource::ScreenShare,
            track::TrackSource::ScreenshareAudio => RoomTrackSource::ScreenShareAudio,
            track::TrackSource::Unknown => RoomTrackSource::Unknown,
        };

        let participant = self
            .room_info
            .participants
            .iter_mut()
            .find(|p| p.identity == participant.identity().as_str() && p.left_at.is_none());
        if let Some(p) = participant {
            p.tracks.push(RoomTrack {
                sid: track_sid,
                name: Some(track_name.to_string()),
                track_kind,
                track_source,
            });
        }
    }

    fn mark_room_ended(&mut self) {
        let left_at_timestamp = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
        self.room_info
            .participants
            .iter_mut()
            .filter(|p| p.left_at.is_none())
            .for_each(|p| {
                p.left_at = Some(left_at_timestamp);
            });
    }
}

#[derive(Debug, Error)]
pub enum RoomListenerError {
    #[error("{0}")]
    ConnectionError(String),
}

pub async fn listen(server_url: &str, token: &str) -> Result<RoomInfo, RoomListenerError> {
    let (room, mut room_events) = Room::connect(server_url, token, RoomOptions::default())
        .await
        .map_err(|err| {
            RoomListenerError::ConnectionError(format!("Failed to connect to room: {}", err))
        })?;
    let mut room_info_capturer = RoomInfoCapturer::new(&room.name());
    while let Some(event) = room_events.recv().await {
        match event {
            RoomEvent::TrackSubscribed {
                track,
                publication: _,
                participant,
            } => {
                room_info_capturer.mark_track_published(track, participant);
            }
            RoomEvent::ParticipantConnected(participant) => {
                room_info_capturer.mark_participant_joined(participant);
            }
            RoomEvent::ParticipantDisconnected(participant) => {
                room_info_capturer.mark_participant_left(participant);
            }
            RoomEvent::Disconnected { reason } => {
                room_info_capturer.mark_room_ended();
                log::info!("Disconnected from room: {:?}", reason);
                break;
            }
            _ => {}
        }
    }
    Ok(room_info_capturer.into())
}
