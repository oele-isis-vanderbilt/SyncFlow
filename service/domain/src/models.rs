use crate::schema::{login_sessions, users};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Role"]
#[DbValueStyle = "UPPERCASE"]
pub enum Role {
    ADMIN,
    USER,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::ADMIN => write!(f, "ADMIN"),
            Role::USER => write!(f, "USER"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,

    #[diesel(column_name = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,

    #[diesel(column_name = "updatedAt")]
    pub updated_at: Option<chrono::NaiveDateTime>,

    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = login_sessions)]
pub struct LoginSession {
    pub session_id: Uuid,
    pub user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = login_sessions)]
pub struct NewLoginSession {
    pub user_id: i32,
}
