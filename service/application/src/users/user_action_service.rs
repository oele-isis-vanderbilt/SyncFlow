use std::sync::Arc;
use infrastructure::DbPool;
use domain::models::{CreateRoomAction, NewCreateRoomAction, ListRoomAction, NewListRoomsAction, DeleteRoomAction, NewDeleteRoomAction, GenerateTokenAction, NewGenerateTokenAction};
use diesel::prelude::*;
use domain::schema::list_rooms_actions::dsl::list_rooms_actions;


#[derive(Debug)]
pub struct UserActionRegisterer {
    pool: Arc<DbPool>,
}

impl UserActionRegisterer {
    pub fn new(pool: Arc<DbPool>) -> Self {
        UserActionRegisterer {
            pool,
        }
    }

    pub fn register_create_room(&self, new_create_room_action: NewCreateRoomAction) -> QueryResult<CreateRoomAction> {
        use domain::schema::create_room_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let result: QueryResult<CreateRoomAction> = diesel::insert_into(create_room_actions)
            .values(&new_create_room_action)
            .returning(create_room_actions::all_columns)
            .get_result(&mut conn);

        result
    }

    pub fn register_list_rooms(&self, new_list_rooms_action: NewListRoomsAction) -> QueryResult<ListRoomAction> {
        use domain::schema::list_rooms_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let result: QueryResult<ListRoomAction> = diesel::insert_into(list_rooms_actions)
            .values(&new_list_rooms_action)
            .returning(list_rooms_actions::all_columns)
            .get_result(&mut conn);

        result
    }

    pub fn register_generate_token(&self, new_generate_token_action: NewGenerateTokenAction) -> QueryResult<GenerateTokenAction> {
        use domain::schema::generate_token_actions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let new_generate_token_action = NewGenerateTokenAction {
            user_id,
            token_identity,
            token_room,
        };
        let result: QueryResult<GenerateTokenAction> = diesel::insert_into(generate_token_actions)
            .values(&new_generate_token_action)
            .returning(generate_token_actions::all_columns)
            .get_result(&mut conn);

        result
    }

}

impl Clone for UserActionRegisterer {
    fn clone(&self) -> Self {
        UserActionRegisterer {
            pool: self.pool.clone(),
        }
    }
}