// @generated automatically by Diesel CLI.

pub mod syncflow {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "KeyType", schema = "syncflow"))]
        pub struct KeyType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "project_session_status", schema = "syncflow"))]
        pub struct ProjectSessionStatus;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "session_egress_status", schema = "syncflow"))]
        pub struct SessionEgressStatus;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "session_egress_type", schema = "syncflow"))]
        pub struct SessionEgressType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "StorageType", schema = "syncflow"))]
        pub struct StorageType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "track_kind", schema = "syncflow"))]
        pub struct TrackKind;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "track_source", schema = "syncflow"))]
        pub struct TrackSource;
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
        syncflow.login_sessions (session_id) {
            session_id -> Uuid,
            user_id -> Int4,
            created_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::TrackKind;
        use super::sql_types::TrackSource;

        syncflow.participant_tracks (id) {
            id -> Uuid,
            #[max_length = 255]
            sid -> Varchar,
            #[max_length = 255]
            name -> Nullable<Varchar>,
            kind -> TrackKind,
            source -> TrackSource,
            participant_id -> Uuid,
        }
    }

    diesel::table! {
        syncflow.project_api_keys (id) {
            id -> Int4,
            api_key -> Text,
            api_secret -> Text,
            comments -> Nullable<Text>,
            created_at -> Timestamp,
            user_id -> Int4,
            project_id -> Uuid,
        }
    }

    diesel::table! {
        syncflow.project_devices (id) {
            id -> Uuid,
            #[max_length = 50]
            device_name -> Varchar,
            #[max_length = 50]
            device_group -> Varchar,
            comments -> Nullable<Text>,
            registered_at -> Timestamp,
            project_id -> Uuid,
            registered_by -> Int4,
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
            stopped_at -> Nullable<Timestamptz>,
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
        use diesel::sql_types::*;
        use super::sql_types::SessionEgressType;
        use super::sql_types::SessionEgressStatus;

        syncflow.session_egresses (id) {
            id -> Uuid,
            #[max_length = 50]
            track_id -> Varchar,
            #[max_length = 50]
            egress_id -> Varchar,
            started_at -> Int8,
            egress_type -> Nullable<SessionEgressType>,
            status -> SessionEgressStatus,
            destination -> Nullable<Text>,
            #[max_length = 250]
            room_name -> Varchar,
            session_id -> Uuid,
            participant_id -> Nullable<Uuid>,
            db_track_id -> Nullable<Uuid>,
        }
    }

    diesel::table! {
        syncflow.session_participants (id) {
            id -> Uuid,
            #[max_length = 255]
            participant_identity -> Varchar,
            #[max_length = 255]
            participant_name -> Varchar,
            joined_at -> Int8,
            left_at -> Nullable<Int8>,
            session_id -> Uuid,
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
    diesel::joinable!(login_sessions -> users (user_id));
    diesel::joinable!(participant_tracks -> session_participants (participant_id));
    diesel::joinable!(project_api_keys -> projects (project_id));
    diesel::joinable!(project_api_keys -> users (user_id));
    diesel::joinable!(project_devices -> projects (project_id));
    diesel::joinable!(project_devices -> users (registered_by));
    diesel::joinable!(project_sessions -> projects (project_id));
    diesel::joinable!(projects -> users (user_id));
    diesel::joinable!(session_egresses -> participant_tracks (db_track_id));
    diesel::joinable!(session_egresses -> project_sessions (session_id));
    diesel::joinable!(session_egresses -> session_participants (participant_id));
    diesel::joinable!(session_participants -> project_sessions (session_id));

    diesel::allow_tables_to_appear_in_same_query!(
        api_keys,
        login_sessions,
        participant_tracks,
        project_api_keys,
        project_devices,
        project_sessions,
        projects,
        session_egresses,
        session_participants,
        users,
    );
}
