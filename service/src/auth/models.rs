use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::ToSql;

use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::*;

#[derive(Debug, Clone, Copy, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Role"]
#[DbValueStyle = "UPPERCASE"]
pub enum Role {
    User,
    Admin,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,

    #[diesel(column_name = "createdAt")]
    pub created_at: NaiveDateTime,

    #[diesel(column_name = "updatedAt")]
    pub updated_at: NaiveDateTime,
    pub role: Role,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,

    #[diesel(column_name = "createdAt")]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: NaiveDateTime,
}
