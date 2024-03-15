use crate::livekit::room::RoomService;
use crate::mmla::user_actions::UserActions;
use domain::models::NewCreateRoomAction;
use shared::livekit_models::{CreateRoomRequest, LivekitRoom};
use std::fmt::Display;

#[derive(Debug)]
pub enum ServiceError {
    RoomCreationError(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::RoomCreationError(e) => write!(f, "RoomCreationError: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MMLAService {
    room_service: RoomService,
    user_actions: UserActions,
}

impl MMLAService {
    pub fn new(room_service: RoomService, user_actions: UserActions) -> Self {
        MMLAService {
            room_service,
            user_actions,
        }
    }

    pub async fn create_room(
        &self,
        user_id: i32,
        create_room_request: CreateRoomRequest,
    ) -> Result<LivekitRoom, ServiceError> {
        let create_room_result = self
            .room_service
            .create_room(&create_room_request.name, create_room_request.options)
            .await;

        if let Ok(room) = create_room_result {
            let new_create_room_action = NewCreateRoomAction {
                user_id,
                room_name: room.name.clone(),
            };

            let _ = self
                .user_actions
                .register_create_room(new_create_room_action);

            Ok(LivekitRoom::from(room))
        } else {
            Err(ServiceError::RoomCreationError(
                "Error creating room".to_string(),
            ))
        }
    }
}
