use crate::livekit::egress::EgressService;
use crate::livekit::room::RoomService;
use crate::livekit::token::create_token;
use crate::mmla::user_actions::UserActions;
use crate::mmla::utils::{get_track_egress_destination, get_track_egress_destination_path};
use domain::models::{
    EgressType, NewCreateRoomAction, NewDeleteRoomAction, NewGenerateTokenAction,
    NewListRoomsAction, NewUserEgressAction,
};
use futures::stream::{self, StreamExt};
use livekit_api::access_token::AccessTokenError;
use livekit_api::services::ServiceError;
use livekit_protocol::{EgressInfo, EgressStatus, ParticipantInfo};
use shared::livekit_models::{CreateRoomRequest, LivekitRoom, TokenRequest, TokenResponse};
use shared::response_models::Response;
use thiserror::Error;

use super::user_actions::UserActionError;

#[derive(Debug, Error)]
pub enum MMLAServiceError {
    #[error("Livekit Error: {0}")]
    LiveKitError(#[from] ServiceError),
    #[error("User Action Error: {0}")]
    UserActionError(#[from] UserActionError),
    #[error("Room Not Found Error: {0}")]
    RoomNotFoundError(String),
    #[error("Access Token Error: {0}")]
    AccessTokenError(#[from] AccessTokenError),
}

impl From<MMLAServiceError> for Response {
    fn from(val: MMLAServiceError) -> Self {
        match val {
            MMLAServiceError::LiveKitError(e) => Response {
                status: 500,
                message: e.to_string(),
            },
            MMLAServiceError::UserActionError(e) => Response {
                status: 500,
                message: e.to_string(),
            },
            MMLAServiceError::RoomNotFoundError(e) => Response {
                status: 404,
                message: e,
            },
            MMLAServiceError::AccessTokenError(e) => Response {
                status: 500,
                message: e.to_string(),
            },
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
    ) -> Result<LivekitRoom, MMLAServiceError> {
        let room = self
            .room_service
            .create_room(&create_room_request.name, create_room_request.options)
            .await?;

        let new_create_room_action = NewCreateRoomAction {
            user_id,
            room_name: room.name.clone(),
        };

        let _ = self
            .user_actions
            .register_create_room(new_create_room_action);

        Ok(LivekitRoom::from(room))
    }

    async fn find_user_created_room(&self, user_id: i32, room_name: &str) -> Option<LivekitRoom> {
        let user_rooms = self.user_actions.list_created_rooms(user_id).ok()?;
        let active_rooms = self.room_service.list_rooms(None).await.ok()?;
        active_rooms
            .iter()
            .find(|room| {
                room.name == room_name && user_rooms.iter().any(|r| r.room_name == room_name)
            })
            .map(|r| LivekitRoom::from(r.clone()))
    }

    pub async fn delete_room(
        &self,
        user_id: i32,
        room_name: String,
    ) -> Result<LivekitRoom, MMLAServiceError> {
        let room = self
            .find_user_created_room(user_id, &room_name)
            .await
            .ok_or_else(|| MMLAServiceError::RoomNotFoundError(room_name.clone()))?;

        self.room_service.delete_room(&room.name).await?;
        let new_delete_room_action = NewDeleteRoomAction {
            user_id,
            room_name: room.name.clone(),
        };

        let _ = self
            .user_actions
            .register_delete_room(new_delete_room_action);

        Ok(room)
    }

    pub async fn list_rooms(&self, user_id: i32) -> Result<Vec<LivekitRoom>, MMLAServiceError> {
        let active_rooms = self.room_service.list_rooms(None).await?;

        let collected_rooms: Vec<LivekitRoom> = stream::iter(active_rooms)
            .filter_map(
                |room| async move { self.find_user_created_room(user_id, &room.name).await },
            )
            .collect()
            .await;

        let new_list_rooms_action = NewListRoomsAction { user_id };

        let _ = self.user_actions.register_list_rooms(new_list_rooms_action);

        Ok(collected_rooms)
    }

    pub async fn generate_token(
        &self,
        user_id: i32,
        token_request: TokenRequest,
        api_key: String,
        api_secret: String,
    ) -> Result<TokenResponse, MMLAServiceError> {
        let room_name = &token_request.video_grants.room;
        if !token_request.video_grants.room_create {
            self.find_user_created_room(user_id, room_name)
                .await
                .ok_or_else(|| MMLAServiceError::RoomNotFoundError(room_name.clone()))?;
        }

        let token = create_token(&token_request, &api_key, &api_secret)
            .map(|token| TokenResponse::new(token, token_request.identity.clone()))?;

        let _ = self
            .user_actions
            .register_generate_token(NewGenerateTokenAction {
                token_identity: token_request.identity.clone(),
                user_id,
                token_room: room_name.clone(),
            });

        Ok(token)
    }

    pub async fn list_participants(
        &self,
        user_id: i32,
        room_name: &str,
    ) -> Result<Vec<ParticipantInfo>, MMLAServiceError> {
        // self.room_service.list_participants(room_name).await
        let room = self
            .find_user_created_room(user_id, room_name)
            .await
            .ok_or_else(|| MMLAServiceError::RoomNotFoundError(room_name.to_string()))?;

        let participants = self.room_service.list_participants(&room.name).await?;

        Ok(participants)
    }

    pub async fn list_egresses(
        &self,
        user_id: i32,
        room_name: &str,
    ) -> Result<Vec<EgressInfo>, MMLAServiceError> {
        let room = self
            .find_user_created_room(user_id, room_name)
            .await
            .ok_or_else(|| MMLAServiceError::RoomNotFoundError(room_name.to_string()))?;

        let egresses = self.egress_service.list_egresses(&room.name).await?;

        Ok(egresses)
    }

    pub async fn record_track(
        &self,
        user_id: i32,
        room_name: &str,
        track_id: &str,
    ) -> Result<EgressInfo, MMLAServiceError> {
        let room = self
            .find_user_created_room(user_id, room_name)
            .await
            .ok_or_else(|| MMLAServiceError::RoomNotFoundError(room_name.to_string()))?;

        let result = self
            .egress_service
            .start_local_track_egress(&room.name, track_id)
            .await?;

        let egress_destination = get_track_egress_destination(result.request.clone());
        let filepath = get_track_egress_destination_path(result.result.clone());

        if filepath.is_some() && egress_destination.is_some() {
            let new_user_egress_action = NewUserEgressAction {
                user_id,
                room_name: room_name.to_string(),
                egress_id: result.egress_id.clone(),
                egress_type: EgressType::Track,
                egress_destination_root: self.egress_service.get_egress_root(),
                egress_destination: egress_destination.unwrap(),
                egress_destination_path: filepath.unwrap(),
                updated_at: None,
                success: false,
            };
            let _ = self.user_actions.register_egress(new_user_egress_action);
        }

        Ok(result)
    }

    pub async fn stop_recording(
        &self,
        user_id: i32,
        room_name: &str,
        egress_id: &str,
    ) -> Result<EgressInfo, MMLAServiceError> {
        let _ = self
            .find_user_created_room(user_id, room_name)
            .await
            .ok_or_else(|| MMLAServiceError::RoomNotFoundError(room_name.to_string()))?;

        let egress_result = self.egress_service.stop_egress(egress_id).await?;

        let egress_destination = get_track_egress_destination(egress_result.request.clone());
        let filepath = get_track_egress_destination_path(egress_result.result.clone());

        if filepath.is_some() && egress_destination.is_some() {
            let new_user_egress_action = NewUserEgressAction {
                user_id,
                room_name: room_name.to_string(),
                egress_id: egress_result.egress_id.clone(),
                egress_type: EgressType::Track,
                egress_destination_root: self.egress_service.get_egress_root(),
                egress_destination: egress_destination.unwrap(),
                egress_destination_path: filepath.unwrap(),
                updated_at: Some(chrono::Local::now().naive_local()),
                success: EgressStatus::EgressComplete as i32 == egress_result.status
                    || EgressStatus::EgressEnding as i32 == egress_result.status,
            };
            let _ = self.user_actions.update_egress(new_user_egress_action);
        }

        Ok(egress_result)
    }
}
