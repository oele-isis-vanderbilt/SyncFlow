// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "Role"))]
    pub struct Role;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        role -> Role,
    }
}
