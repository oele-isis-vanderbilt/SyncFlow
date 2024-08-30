// @generated automatically by Diesel CLI.

pub mod syncflow {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "EgressDestination", schema = "syncflow"))]
        pub struct EgressDestination;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "EgressType", schema = "syncflow"))]
        pub struct EgressType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "KeyType", schema = "syncflow"))]
        pub struct KeyType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "project_session_status", schema = "syncflow"))]
        pub struct ProjectSessionStatus;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "StorageType", schema = "syncflow"))]
        pub struct StorageType;
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::KeyType;

        syncflow.api_keys (id) {
            id -> Int4,
            #[max_length = 255]
            key -> Varchar,
            user_id -> Int4,
            #[max_length = 255]
            secret -> Varchar,
            created_at -> Nullable<Timestamptz>,
            valid -> Bool,
            comment -> Nullable<Text>,
            key_type -> KeyType,
        }
    }

    diesel::table! {
        syncflow.create_room_actions (id) {
            id -> Int4,
            room_name -> Text,
            user_id -> Int4,
            created_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        syncflow.delete_room_actions (id) {
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

        syncflow.egress_actions (id) {
            id -> Int4,
            user_id -> Int4,
            egress_id -> Text,
            room_name -> Text,
            egress_destination_path -> Text,
            egress_destination_root -> Text,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            success -> Bool,
            egress_destination -> EgressDestination,
            egress_type -> EgressType,
        }
    }

    diesel::table! {
        syncflow.generate_token_actions (id) {
            id -> Int4,
            user_id -> Int4,
            token_identity -> Text,
            token_room -> Text,
            generated_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        syncflow.list_rooms_actions (id) {
            id -> Int4,
            user_id -> Int4,
            listed_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        syncflow.login_sessions (session_id) {
            session_id -> Uuid,
            user_id -> Int4,
            created_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::ProjectSessionStatus;

        syncflow.project_sessions (id) {
            id -> Uuid,
            #[max_length = 50]
            name -> Varchar,
            comments -> Nullable<Text>,
            empty_timeout -> Int4,
            max_participants -> Int4,
            #[max_length = 50]
            livekit_room_name -> Varchar,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            status -> ProjectSessionStatus,
            project_id -> Uuid,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::StorageType;

        syncflow.projects (id) {
            id -> Uuid,
            user_id -> Int4,
            #[max_length = 50]
            name -> Varchar,
            description -> Nullable<Text>,
            livekit_server_url -> Text,
            livekit_server_api_key -> Text,
            livekit_server_api_secret -> Text,
            storage_type -> StorageType,
            #[max_length = 50]
            bucket_name -> Varchar,
            endpoint -> Text,
            access_key -> Text,
            secret_key -> Text,
            #[max_length = 50]
            region -> Nullable<Varchar>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        syncflow.users (id) {
            id -> Int4,
            #[max_length = 255]
            username -> Varchar,
            #[max_length = 255]
            email -> Varchar,
            password -> Nullable<Text>,
            createdAt -> Nullable<Timestamptz>,
            updatedAt -> Nullable<Timestamptz>,
            #[max_length = 255]
            oauth_provider -> Nullable<Varchar>,
            #[max_length = 255]
            oauth_provider_user_id -> Nullable<Varchar>,
            #[max_length = 255]
            first_name -> Nullable<Varchar>,
            #[max_length = 255]
            middle_name -> Nullable<Varchar>,
            #[max_length = 255]
            last_name -> Nullable<Varchar>,
            #[max_length = 255]
            organization -> Nullable<Varchar>,
            #[max_length = 255]
            job_role -> Nullable<Varchar>,
        }
    }

    diesel::joinable!(api_keys -> users (user_id));
    diesel::joinable!(create_room_actions -> users (user_id));
    diesel::joinable!(delete_room_actions -> users (user_id));
    diesel::joinable!(egress_actions -> users (user_id));
    diesel::joinable!(generate_token_actions -> users (user_id));
    diesel::joinable!(list_rooms_actions -> users (user_id));
    diesel::joinable!(login_sessions -> users (user_id));
    diesel::joinable!(project_sessions -> projects (project_id));
    diesel::joinable!(projects -> users (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        api_keys,
        create_room_actions,
        delete_room_actions,
        egress_actions,
        generate_token_actions,
        list_rooms_actions,
        login_sessions,
        project_sessions,
        projects,
        users,
    );
}
