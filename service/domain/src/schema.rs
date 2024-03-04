// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    login_sessions (session_id) {
        session_id -> Uuid,
        user_id -> Int4,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password -> Text,
        createdAt -> Nullable<Timestamptz>,
        updatedAt -> Nullable<Timestamptz>,
        role -> Role,
    }
}

diesel::joinable!(login_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(login_sessions, users,);
