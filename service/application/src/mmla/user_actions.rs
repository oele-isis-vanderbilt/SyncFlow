use diesel::prelude::*;
use domain::models::{
    CreateRoomAction, DeleteRoomAction, GenerateTokenAction, ListRoomsAction, NewCreateRoomAction,
    NewDeleteRoomAction, NewGenerateTokenAction, NewListRoomsAction, NewUserEgressAction,
    UserEgressAction,
};
use infrastructure::DbPool;
use log::{error, info};
use std::sync::Arc;

pub enum UserActionError {
    RoomCreationError(String),
    DatabaseError(String),
}

#[derive(Debug, Clone)]
pub struct UserActions {
    pool: Arc<DbPool>,
}

impl UserActions {
    pub fn new(pool: Arc<DbPool>) -> Self {
        UserActions { pool }
    }

    pub fn register_create_room(
        &self,
        new_create_room_action: NewCreateRoomAction,
    ) -> Result<CreateRoomAction, UserActionError> {
        use domain::schema::create_room_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(create_room_actions)
            .values(&new_create_room_action)
            .get_result::<CreateRoomAction>(&mut conn)
            .map(|action| {
                info!("Created room action: {:?}", action);
                action
            })
            .map_err(|e| {
                error!("Error registering create room action: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            })
    }

    pub fn register_delete_room(
        &self,
        new_delete_room_action: NewDeleteRoomAction,
    ) -> Result<DeleteRoomAction, UserActionError> {
        use domain::schema::delete_room_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(delete_room_actions)
            .values(&new_delete_room_action)
            .get_result::<DeleteRoomAction>(&mut conn)
            .map_err(|e| {
                error!("Error registering delete room action: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            })
    }

    pub fn register_list_rooms(
        &self,
        new_list_room_action: NewListRoomsAction,
    ) -> Result<ListRoomsAction, UserActionError> {
        use domain::schema::list_rooms_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(list_rooms_actions)
            .values(&new_list_room_action)
            .get_result::<ListRoomsAction>(&mut conn)
            .map_err(|e| {
                error!("Error registering list room action: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            })
    }

    pub fn list_created_rooms(&self, uid: i32) -> Result<Vec<CreateRoomAction>, UserActionError> {
        use domain::schema::create_room_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        create_room_actions
            .filter(user_id.eq(uid))
            .load::<CreateRoomAction>(&mut conn)
            .map_err(|e| {
                error!("Error listing created rooms: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            })
    }

    pub fn register_generate_token(
        &self,
        new_generate_token_action: NewGenerateTokenAction,
    ) -> Result<GenerateTokenAction, UserActionError> {
        use domain::schema::generate_token_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(generate_token_actions)
            .values(&new_generate_token_action)
            .get_result::<GenerateTokenAction>(&mut conn)
            .map_err(|e| {
                error!("Error registering generate token action: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            })
    }

    pub fn register_egress(
        &self,
        new_egress_action: NewUserEgressAction,
    ) -> Result<UserEgressAction, UserActionError> {
        use domain::schema::egress_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(egress_actions)
            .values(&new_egress_action)
            .get_result::<UserEgressAction>(&mut conn)
            .map_err(|e| {
                error!("Error registering egress action: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            })
    }

    pub fn update_egress(
        &self,
        action: NewUserEgressAction,
    ) -> Result<UserEgressAction, UserActionError> {
        use domain::schema::egress_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::update(egress_actions.filter(egress_id.eq(&action.egress_id)))
            .set(&action)
            .get_result::<UserEgressAction>(&mut conn)
            .map_err(|e| {
                error!("Error updating egress action: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            })
    }
}
