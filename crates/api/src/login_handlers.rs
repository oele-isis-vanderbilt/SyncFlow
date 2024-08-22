use crate::helpers::{error_response, json_ok_response};
use actix_web::web::{Json, ReqData};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use application::users::account_service::AccountService;
use application::users::tokens_manager::UserInfo;
use shared::constants;
use shared::response_models::Response;
use shared::user_models::{ApiKeyRequest, LoginRequest, RefreshTokenRequest, SignUpRequest};

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
    user_auth
        .login(login_request.into_inner())
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    post,
    path = "/users/signup",
    request_body = SignUpRequest,
    responses(
        (status = 200, description = "User created succesfully", body =()),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    )
)]
#[post("/signup")]
pub async fn signup(
    user_auth: web::Data<AccountService>,
    signup_request: Json<SignUpRequest>,
) -> HttpResponse {
    user_auth
        .signup(signup_request.into_inner())
        .map(json_ok_response)
        .unwrap_or_else(error_response)
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

#[utoipa::path(
    post,
    path = "/users/refresh-token",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = bool),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    )
)]
#[post("/refresh-token")]
pub async fn refresh_login_token(
    user_auth: web::Data<AccountService>,
    refresh_request: Json<RefreshTokenRequest>,
) -> HttpResponse {
    user_auth
        .refresh_token(refresh_request.into_inner())
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[post("/api-key")]
pub async fn create_api_key(
    api_token_request: Json<ApiKeyRequest>,
    user_data: ReqData<UserInfo>,
    user_auth: web::Data<AccountService>,
) -> HttpResponse {
    user_auth
        .generate_api_keys(
            user_data.into_inner().user_id,
            &api_token_request.into_inner(),
        )
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[delete("/api-key/{key_id}")]
pub async fn delete_api_key(
    account_service: web::Data<AccountService>,
    key_id: web::Path<String>,
    user_data: ReqData<UserInfo>,
) -> HttpResponse {
    account_service
        .delete_api_key(user_data.into_inner().user_id, &key_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
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
    user_data: ReqData<UserInfo>,
    user_auth: web::Data<AccountService>,
) -> HttpResponse {
    user_auth
        .list_api_keys(user_data.into_inner().user_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Get the user", body = User),
        (status = 401, description = "Invalid Token")
    )
)]
#[get("/me")]
pub async fn me(
    user_data: ReqData<UserInfo>,
    user_auth: web::Data<AccountService>,
) -> HttpResponse {
    user_auth
        .get_user(user_data.into_inner().user_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let users_scope = web::scope("/users")
        .service(login)
        .service(logout)
        .service(refresh_login_token)
        .service(create_api_key)
        .service(delete_api_key)
        .service(list_all_api_keys)
        .service(signup)
        .service(me);
    cfg.service(users_scope);
}
