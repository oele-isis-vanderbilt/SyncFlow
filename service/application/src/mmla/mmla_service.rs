use crate::livekit::egress::EgressService;
use crate::livekit::room::RoomService;
use crate::livekit::token::create_token;
use crate::mmla::user_actions::UserActions;
use domain::models::{
    NewCreateRoomAction, NewDeleteRoomAction, NewGenerateTokenAction, NewListRoomsAction,
};
use livekit_protocol::{EgressInfo, ParticipantInfo};
use shared::livekit_models::{CreateRoomRequest, LivekitRoom, TokenRequest, TokenResponse};
use shared::response_models::Response;
use std::fmt::Display;

#[derive(Debug)]
pub enum ServiceError {
    RoomCreationError(String),
    DeleteRoomError(String),
    RoomListError(String),
    EgressError(String),
    PermissionError(String),
    AccessTokenError(String),
}

impl Into<Response> for ServiceError {
    fn into(self) -> Response {
        match self {
            ServiceError::RoomCreationError(e) => Response {
                status: 500,
                message: e,
            },
            ServiceError::DeleteRoomError(e) => Response {
                status: 500,
                message: e,
            },
            ServiceError::PermissionError(e) => Response {
                status: 403,
                message: e,
            },
            ServiceError::RoomListError(e) => Response {
                status: 500,
                message: e,
            },
            ServiceError::AccessTokenError(e) => Response {
                status: 500,
                message: e,
            },
            ServiceError::EgressError(e) => Response {
                status: 500,
                message: e,
            },
        }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::RoomCreationError(e) => write!(f, "RoomCreationError: {}", e),
            ServiceError::DeleteRoomError(e) => write!(f, "DeleteRoomError: {}", e),
            ServiceError::PermissionError(e) => write!(f, "PermissionError: {}", e),
            ServiceError::RoomListError(e) => write!(f, "RoomListError: {}", e),
            ServiceError::AccessTokenError(e) => write!(f, "AccessTokenError: {}", e),
            ServiceError::EgressError(e) => write!(f, "EgressListError: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MMLAService {
    room_service: RoomService,
    egress_service: EgressService,
    user_actions: UserActions,
}

impl MMLAService {
    pub fn new(
        room_service: RoomService,
        egress_service: EgressService,
        user_actions: UserActions,
    ) -> Self {
        MMLAService {
            room_service,
            user_actions,
            egress_service,
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

    pub async fn delete_room(
        &self,
        user_id: i32,
        room_name: String,
    ) -> Result<Response, ServiceError> {
        let user_rooms = self.user_actions.list_created_rooms(user_id);

        if let Ok(rooms) = user_rooms {
            if rooms.iter().any(|room| room.room_name == room_name) {
                let delete_room_result = self.room_service.delete_room(&room_name).await;

                if let Ok(_) = delete_room_result {
                    let new_room_delete_actions = NewDeleteRoomAction {
                        user_id,
                        room_name: room_name.clone(),
                    };

                    let _ = self
                        .user_actions
                        .register_delete_room(new_room_delete_actions);

                    Ok(Response {
                        message: format!("Room {} deleted successfully", room_name),
                        status: 200,
                    })
                } else {
                    Err(ServiceError::DeleteRoomError(format!(
                        "Room {} deleted successfully",
                        room_name
                    )))
                }
            } else {
                Err(ServiceError::DeleteRoomError(format!(
                    "Room {} not found",
                    room_name
                )))
            }
        } else {
            Err(ServiceError::PermissionError(
                "Permission denied".to_string(),
            ))
        }
    }

    pub async fn list_rooms(&self, user_id: i32) -> Result<Vec<LivekitRoom>, ServiceError> {
        let user_rooms = self.user_actions.list_created_rooms(user_id);

        if let Ok(rooms) = user_rooms {
            let room_names = rooms.iter().map(|room| room.room_name.clone()).collect();
            let livekit_rooms = self.room_service.list_rooms(Some(room_names)).await;

            if let Ok(rooms) = livekit_rooms {
                let new_list_rooms_action = NewListRoomsAction { user_id };
                let _ = self.user_actions.register_list_rooms(new_list_rooms_action);

                Ok(rooms.into_iter().map(LivekitRoom::from).collect())
            } else {
                Err(ServiceError::RoomListError(
                    "Error listing rooms".to_string(),
                ))
            }
        } else {
            Err(ServiceError::PermissionError(
                "Permission denied".to_string(),
            ))
        }
    }

    pub async fn generate_token(
        &self,
        user_id: i32,
        token_request: TokenRequest,
        api_key: String,
        api_secret: String,
    ) -> Result<TokenResponse, ServiceError> {
        let room_name = token_request.video_grants.room.clone();
        let can_create_room = token_request.video_grants.room_create;
        if !can_create_room {
            let user_rooms = self.user_actions.list_created_rooms(user_id);

            if let Ok(rooms) = user_rooms {
                if rooms.iter().any(|room| room.room_name == room_name) {
                    create_token(&token_request, api_key, api_secret)
                        .map(|t| {
                            let _ =
                                self.user_actions
                                    .register_generate_token(NewGenerateTokenAction {
                                        user_id,
                                        token_identity: token_request.identity.clone(),
                                        token_room: room_name.clone(),
                                    });
                            TokenResponse::new(t, token_request.identity.clone())
                        })
                        .map_err(|e| ServiceError::AccessTokenError(e.to_string()))
                } else {
                    Err(ServiceError::PermissionError(
                        "Permission denied".to_string(),
                    ))
                }
            } else {
                Err(ServiceError::PermissionError(
                    "Permission denied".to_string(),
                ))
            }
        } else {
            create_token(&token_request, api_key, api_secret)
                .map_err(|e| ServiceError::AccessTokenError(e.to_string()))
                .map(|t| {
                    let _ = self
                        .user_actions
                        .register_generate_token(NewGenerateTokenAction {
                            user_id,
                            token_identity: token_request.identity.clone(),
                            token_room: room_name.clone(),
                        });

                    TokenResponse::new(t, token_request.identity.clone())
                })
        }
    }

    pub async fn list_participants(
        &self,
        user_id: i32,
        room_name: &str,
    ) -> Result<Vec<ParticipantInfo>, ServiceError> {
        // self.room_service.list_participants(room_name).await
        let user_rooms = self.user_actions.list_created_rooms(user_id);
        if let Ok(rooms) = user_rooms {
            if rooms.iter().any(|room| room.room_name == room_name) {
                self.room_service
                    .list_participants(room_name)
                    .await
                    .map_err(|e| ServiceError::EgressError(e.to_string()))
            } else {
                Err(ServiceError::PermissionError(
                    "Permission denied".to_string(),
                ))
            }
        } else {
            Err(ServiceError::PermissionError(
                "Permission denied".to_string(),
            ))
        }
    }

    pub async fn list_egresses(
        &self,
        user_id: i32,
        room_name: &str,
    ) -> Result<Vec<EgressInfo>, ServiceError> {
        if self.is_user_created_room(user_id, room_name) {
            self.egress_service
                .list_egresses(room_name.into())
                .await
                .map_err(|e| ServiceError::RoomListError(e.to_string()))
        } else {
            Err(ServiceError::PermissionError(
                "Permission denied".to_string(),
            ))
        }
    }

    pub async fn record_track(
        &self,
        user_id: i32,
        room_name: &str,
        track_id: &str,
    ) -> Result<EgressInfo, ServiceError> {
        if self.is_user_created_room(user_id, room_name) {
            self.egress_service
                .start_local_track_egress(room_name, track_id)
                .await
                .map_err(|e| ServiceError::EgressError(e.to_string()))
        } else {
            Err(ServiceError::PermissionError(
                "Permission denied".to_string(),
            ))
        }
    }

    pub async fn stop_recording(
        &self,
        user_id: i32,
        room_name: &str,
        egress_id: &str,
    ) -> Result<EgressInfo, ServiceError> {
        if self.is_user_created_room(user_id, room_name) {
            self.egress_service
                .stop_egress(egress_id)
                .await
                .map_err(|e| ServiceError::EgressError(e.to_string()))
        } else {
            Err(ServiceError::PermissionError(
                "Permission denied".to_string(),
            ))
        }
    }

    fn is_user_created_room(&self, user_id: i32, room_name: &str) -> bool {
        let user_rooms = self.user_actions.list_created_rooms(user_id);
        if let Ok(rooms) = user_rooms {
            rooms.iter().any(|room| room.room_name == room_name)
        } else {
            false
        }
    }
}
