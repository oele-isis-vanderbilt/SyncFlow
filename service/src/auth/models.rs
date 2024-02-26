use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::AsExpression;
use std::io::Write;

use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Role {
    User,
    Admin,
}

impl<DB> ToSql<Text, DB> for Role
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        match *self {
            Role::User => "USER".to_sql(out),
            Role::Admin => "ADMIN".to_sql(out),
        }
    }
}

impl<DB> FromSql<Text, DB> for Role
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "USER" => Ok(Role::User),
            "ADMIN" => Ok(Role::Admin),
            _ => Err("Unrecognized role".into()),
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: Role,
}
