use diesel::prelude::*;
use domain::models::{CreateRoomAction, NewCreateRoomAction};
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

        let action = diesel::insert_into(create_room_actions)
            .values(&new_create_room_action)
            .get_result::<CreateRoomAction>(&mut conn)
            .map(|action| {
                info!("Created room action: {:?}", action);
                action
            })
            .map_err(|e| {
                error!("Error registering create room action: {}", e.to_string());
                UserActionError::DatabaseError(e.to_string())
            });
        action
    }
}
