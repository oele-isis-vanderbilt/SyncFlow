use actix_web::web::{Json, ReqData};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use application::users::account_service::AccountService;
use application::users::tokens_manager::UserInfo;
use shared::constants;
use shared::response_models::Response;
use shared::user_models::{ApiKeyRequest, ApiKeyResponse, LoginRequest, TokenResponse};

#[utoipa::path(
    post,
    path = "/users/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = bool),
        (status = 404, description = "User not found"),
        (status = 401, description = "Password mismatch"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    )
)]
#[post("/login")]
pub async fn login(
    user_auth: web::Data<AccountService>,
    login_request: Json<LoginRequest>,
) -> HttpResponse {
    match user_auth.login(login_request.into_inner()) {
        Ok(token_string) => HttpResponse::Ok().json(TokenResponse::bearer(token_string)),
        Err(e) => {
            let response: Response = e.into();
            response.into()
        }
    }
}

#[utoipa::path(
    post,
    path = "/users/logout",
    responses(
        (status = 200, description = "User logged out successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/logout")]
pub async fn logout(req: HttpRequest, user_auth: web::Data<AccountService>) -> HttpResponse {
    match req.headers().get(constants::AUTHORIZATION_HEADER) {
        Some(header) => {
            let token = header.to_str().unwrap().split(' ').collect::<Vec<&str>>()[1];
            match user_auth.logout(token) {
                Ok(_) => HttpResponse::Ok().json(Response {
                    status: 200,
                    message: "User logged out successfully".to_string(),
                }),
                Err(e) => {
                    let response: Response = e.into();
                    response.into()
                }
            }
        }
        None => (Response {
            status: 401,
            message: constants::MESSAGE_INVALID_TOKEN.to_string(),
        })
        .into(),
    }
}

#[post("/api-key")]
pub async fn create_api_key(
    api_token_request: Json<ApiKeyRequest>,
    user_data: Option<ReqData<UserInfo>>,
    user_auth: web::Data<AccountService>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let user_id = user_info.user_id;
            let api_key =
                user_auth.generate_api_keys(user_id, Some(api_token_request.comment.clone()));
            match api_key {
                Ok(key) => {
                    let decrypted = user_auth.decrypt_secret(&key.secret);
                    match decrypted {
                        Ok(secret) => {
                            let key_response = ApiKeyResponse {
                                key: key.key,
                                secret,
                                created_at: key
                                    .created_at
                                    .map(|c| c.and_utc().timestamp() as usize)
                                    .unwrap_or_default(),
                                comment: key.comment.unwrap_or_default(),
                            };
                            HttpResponse::Ok().json(key_response)
                        }
                        Err(e) => {
                            let response: Response = e.into();
                            response.into()
                        }
                    }
                }
                Err(e) => {
                    let response: Response = e.into();
                    response.into()
                }
            }
        }
        None => {
            let response = Response {
                status: 401,
                message: constants::MESSAGE_INVALID_TOKEN.to_string(),
            };
            response.into()
        }
    }
}

#[delete("/api-key/{key_id}")]
pub async fn delete_api_key(
    account_service: web::Data<AccountService>,
    key_id: web::Path<String>,
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let user_id = user_info.user_id;
            let key_id = key_id.into_inner();
            let result = account_service.delete_api_key(user_id, &key_id);
            match result {
                Ok(api_key_resp) => HttpResponse::Ok().json(api_key_resp),
                Err(e) => {
                    let response: Response = e.into();
                    response.into()
                }
            }
        }
        None => {
            let response = Response {
                status: 401,
                message: constants::MESSAGE_INVALID_TOKEN.to_string(),
            };
            response.into()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api-keys",
    responses(
        (status = 200, description = "List of API Keys", body = Vec<ApiKeyResponseWithoutSecret>),
        (status = 401, description = "Invalid Token")
    )
)]
#[get("/api-keys")]
pub async fn list_all_api_keys(
    user_data: Option<ReqData<UserInfo>>,
    user_auth: web::Data<AccountService>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let user_id = user_info.user_id;
            let api_keys = user_auth.list_api_keys(user_id);
            match api_keys {
                Ok(keys) => HttpResponse::Ok().json(keys),
                Err(e) => {
                    let response: Response = e.into();
                    response.into()
                }
            }
        }
        None => {
            let response = Response {
                status: 401,
                message: constants::MESSAGE_INVALID_TOKEN.to_string(),
            };
            response.into()
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let users_scope = web::scope("/users")
        .service(login)
        .service(logout)
        .service(create_api_key)
        .service(delete_api_key)
        .service(list_all_api_keys);
    cfg.service(users_scope);
}
