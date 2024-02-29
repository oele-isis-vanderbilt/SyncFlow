use serde::{Deserialize, Serialize};
use diesel_derive_enum::DbEnum;
use utoipa::ToSchema;
use diesel::prelude::*;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, DbEnum)]
#[db_rename = "UPPERCASE"]
#[ExistingTypePath = "crate::schema::sql_types::Role"]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,

    #[diesel(column_name = "createdAt")]
    pub created_at: chrono::NaiveDateTime,

    #[diesel(column_name = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,

    pub role: Role,
}


