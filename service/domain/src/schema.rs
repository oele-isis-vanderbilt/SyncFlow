// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "EgressDestination"))]
    pub struct EgressDestination;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "EgressType"))]
    pub struct EgressType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    create_room_actions (id) {
        id -> Int4,
        room_name -> Text,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    delete_room_actions (id) {
        id -> Int4,
        room_name -> Text,
        user_id -> Int4,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EgressDestination;
    use super::sql_types::EgressType;

    egress_actions (id) {
        id -> Int4,
        user_id -> Int4,
        egress_id -> Text,
        room_name -> Text,
        egress_destination -> EgressDestination,
        egress_type -> EgressType,
        egress_destination_path -> Text,
        egress_destination_root -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        success -> Bool,
    }
}

diesel::table! {
    generate_token_actions (id) {
        id -> Int4,
        user_id -> Int4,
        token_identity -> Text,
        token_room -> Text,
        generated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    list_rooms_actions (id) {
        id -> Int4,
        user_id -> Int4,
        listed_at -> Nullable<Timestamp>,
    }
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

diesel::joinable!(create_room_actions -> users (user_id));
diesel::joinable!(delete_room_actions -> users (user_id));
diesel::joinable!(egress_actions -> users (user_id));
diesel::joinable!(generate_token_actions -> users (user_id));
diesel::joinable!(list_rooms_actions -> users (user_id));
diesel::joinable!(login_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    create_room_actions,
    delete_room_actions,
    egress_actions,
    generate_token_actions,
    list_rooms_actions,
    login_sessions,
    users,
);
