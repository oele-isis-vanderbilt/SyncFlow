use bcrypt::verify;
use diesel::prelude::*;
use diesel::PgConnection;
use domain::models::{
    ApiKey, KeyType, LoginSession, NewApiKey, NewLoginSession, NewUser, Role, User,
};
use std::fmt::Display;

use crate::users::secret::{encrypt_string, key_secret_pair};
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
    SecretError(String),
    HashError(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::UserNotFound(e) => write!(f, "User not found: {}", e),
            UserError::PasswordMismatch(e) => write!(f, "Password mismatch: {}", e),
            UserError::DatabaseError(e) => write!(f, "Database error: {}", e),
            UserError::TokenExpired(e) => write!(f, "Token expired: {}", e),
            UserError::LoginSessionNotFound(e) => write!(f, "Login session not found: {}", e),
            UserError::TokenError(e) => write!(f, "Token error: {}", e),
            UserError::SecretError(e) => write!(f, "Secret error: {}", e),
            UserError::HashError(e) => write!(f, "Hash error: {}", e),
        }
    }
}

impl From<UserError> for Response {
    fn from(val: UserError) -> Self {
        match val {
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
            UserError::SecretError(e) => Response {
                status: 500,
                message: e,
            },
            UserError::HashError(e) => Response {
                status: 500,
                message: e,
            },
        }
    }
}

fn verify_passwd(password: &str, hash: &str) -> bool {
    let password_match = verify(password, hash);
    password_match.unwrap_or(false)
}

fn generate_hash(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn login(
    login_request: LoginRequest,
    conn: &mut PgConnection,
    encryption_secret: &str,
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

                let login_key = fetch_login_key(usr.id, conn);

                match login_key {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = generate_login_key(usr.id, encryption_secret, conn)?;
                        
                    }
                }

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

pub fn create_user(
    username: &str,
    email: &str,
    password: &str,
    is_admin: bool,
    conn: &mut PgConnection,
) -> Result<User, UserError> {
    let hashed_password =
        generate_hash(password).map_err(|e| UserError::HashError(e.to_string()))?;
    let new_user = NewUser {
        username: username.to_owned(),
        email: email.to_owned(),
        password: hashed_password,
        role: if is_admin { Role::ADMIN } else { Role::USER },
    };

    diesel::insert_into(domain::schema::users::table)
        .values(&new_user)
        .get_result::<User>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}

pub fn user_exists(uname: &str, conn: &mut PgConnection) -> bool {
    use domain::schema::users::dsl::*;

    let user_result = users.filter(username.eq(uname)).first::<User>(conn);
    user_result.is_ok()
}

pub fn generate_login_key(
    uid: i32,
    encryption_key: &str,
    conn: &mut PgConnection,
) -> Result<ApiKey, UserError> {
    generate_api_key(
        uid,
        encryption_key,
        conn,
        KeyType::Login,
        Some("Auto-Generated-Login-Key".to_string()),
    )
}

pub fn generate_non_login_api_key(
    uid: i32,
    encryption_key: &str,
    key_comments: Option<String>,
    conn: &mut PgConnection,
) -> Result<ApiKey, UserError> {
    generate_api_key(uid, encryption_key, conn, KeyType::Api, key_comments)
}

pub fn generate_api_key(
    uid: i32,
    encryption_key: &str,
    conn: &mut PgConnection,
    kt: KeyType,
    key_comments: Option<String>,
) -> Result<ApiKey, UserError> {
    use domain::schema::api_keys::dsl::*;

    let key_secret_pair = key_secret_pair();
    let encrypted_secret_string = encrypt_string(&key_secret_pair.secret, encryption_key)
        .map_err(|e| UserError::SecretError(e.to_string()))?;

    let new_api_key = NewApiKey {
        user_id: uid,
        key: key_secret_pair.key.to_owned(),
        secret: encrypted_secret_string,
        key_type: kt,
        comment: key_comments,
        valid: true,
    };

    diesel::insert_into(api_keys)
        .values(&new_api_key)
        .get_result::<ApiKey>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}

pub fn fetch_login_key(uid: i32, conn: &mut PgConnection) -> Result<ApiKey, UserError> {
    use domain::schema::api_keys::dsl::*;

    api_keys
        .filter(user_id.eq(uid).and(key_type.eq(KeyType::Login)))
        .first::<ApiKey>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}

pub fn fetch_api_key_by_id(key_id: &str, conn: &mut PgConnection) -> Result<ApiKey, UserError> {
    use domain::schema::api_keys::dsl::*;

    api_keys
        .filter(key.eq(key_id))
        .first::<ApiKey>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}

pub fn fetch_api_key_by_key(key_ref: &str, conn: &mut PgConnection) -> Result<ApiKey, UserError> {
    use domain::schema::api_keys::dsl::*;

    api_keys
        .filter(key.eq(key_ref))
        .first::<ApiKey>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}

pub fn get_user(uid: i32, conn: &mut PgConnection) -> Result<User, UserError> {
    use domain::schema::users::dsl::*;

    users
        .filter(id.eq(uid))
        .first::<User>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}

pub fn get_all_api_keys(uid: i32, conn: &mut PgConnection) -> Result<Vec<ApiKey>, UserError> {
    use domain::schema::api_keys::dsl::*;

    api_keys
        .filter(user_id.eq(uid).and(key_type.eq(KeyType::Api)))
        .load::<ApiKey>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}

pub fn delete_api_key(
    uid: i32,
    key_ref: &str,
    conn: &mut PgConnection,
) -> Result<ApiKey, UserError> {
    use domain::schema::api_keys::dsl::*;

    
    diesel::delete(api_keys.filter(user_id.eq(uid).and(key.eq(key_ref))))
        .get_result::<ApiKey>(conn)
        .map_err(|e| UserError::DatabaseError(e.to_string()))
}
