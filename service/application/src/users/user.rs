use bcrypt::verify;
use diesel::prelude::*;
use diesel::PgConnection;
use domain::models::{LoginSession, NewLoginSession, Role, User};

use serde::{Deserialize, Serialize};
use shared::response_models::Response;
use shared::user_models::LoginRequest;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSessionInfo {
    pub session_id: String,
    pub user_id: i32,
    pub user_name: String,
    pub user_role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserError {
    UserNotFound(String),
    PasswordMismatch(String),
    DatabaseError(String),
    TokenExpired(String),
    LoginSessionNotFound(String),
    TokenError(String),
}

impl Into<Response> for UserError {
    fn into(self) -> Response {
        match self {
            UserError::UserNotFound(e) => Response {
                status: 404,
                message: e,
            },
            UserError::PasswordMismatch(e) => Response {
                status: 401,
                message: e,
            },
            UserError::DatabaseError(e) => Response {
                status: 500,
                message: e,
            },
            UserError::TokenExpired(e) => Response {
                status: 401,
                message: e,
            },
            UserError::LoginSessionNotFound(e) => Response {
                status: 404,
                message: e,
            },
            UserError::TokenError(e) => Response {
                status: 401,
                message: e,
            },
        }
    }
}

fn verify_passwd(password: &str, hash: &str) -> bool {
    let password_match = verify(password, hash);
    password_match.unwrap_or(false)
}

pub fn login(
    login_request: LoginRequest,
    conn: &mut PgConnection,
) -> Result<LoginSessionInfo, UserError> {
    use domain::schema::login_sessions::dsl::*;
    use domain::schema::users::dsl::*;

    let (uname, passwd) = (login_request.username_or_email, login_request.password);
    // let user_to_verify = users.filter(username.eq(uname)).first::<User>(conn);
    let user_to_verify = users
        .filter(username.eq(&uname).or(email.eq(&uname)))
        .first::<User>(conn);

    match user_to_verify {
        Ok(usr) => {
            let password_match = verify_passwd(&passwd, &usr.password);
            if password_match {
                let new_login_session = NewLoginSession { user_id: usr.id };

                let new_session = diesel::insert_into(login_sessions)
                    .values(&new_login_session)
                    .get_result::<LoginSession>(conn);

                match new_session {
                    Ok(session) => {
                        let session_info = LoginSessionInfo {
                            session_id: session.session_id.to_string(),
                            user_id: session.user_id,
                            user_name: usr.username,
                            user_role: usr.role,
                        };
                        Ok(session_info)
                    }
                    Err(e) => Err(UserError::DatabaseError(e.to_string())),
                }
            } else {
                Err(UserError::PasswordMismatch("Password Mismatch".to_string()))
            }
        }
        Err(e) => Err(UserError::UserNotFound(e.to_string())),
    }
}

pub fn delete_login_session(sid: &str, conn: &mut PgConnection) -> Result<bool, UserError> {
    use domain::schema::login_sessions::dsl::*;

    let session_uuid = Uuid::parse_str(sid);

    match session_uuid {
        Ok(suuid) => {
            let session_result = login_sessions
                .filter(session_id.eq(suuid))
                .first::<LoginSession>(conn);

            match session_result {
                Ok(_session_info) => {
                    let delete_result =
                        diesel::delete(login_sessions.filter(session_id.eq(suuid))).execute(conn);
                    match delete_result {
                        Ok(_) => Ok(true),
                        Err(e) => Err(UserError::DatabaseError(e.to_string())),
                    }
                }
                Err(e) => Err(UserError::LoginSessionNotFound(e.to_string())),
            }
        }
        Err(e) => Err(UserError::LoginSessionNotFound(e.to_string())),
    }
}

pub fn is_valid_login_session(sid: &str, conn: &mut PgConnection) -> bool {
    use domain::schema::login_sessions::dsl::*;

    let session_uuid = Uuid::parse_str(sid);

    match session_uuid {
        Ok(suuid) => {
            let session_result = login_sessions
                .filter(session_id.eq(suuid))
                .first::<LoginSession>(conn);
            match session_result {
                Ok(_session_info) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
