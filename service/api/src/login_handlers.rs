use actix_web::web::Json;
use actix_web::{post, web, HttpRequest, HttpResponse};
use application::users::account_service::AccountService;
use infrastructure::establish_connection_pool;
use serde_json::json;
use shared::constants;
use shared::user_models::{LoginRequest, TokenResponse};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/users/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = bool),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/login")]
pub async fn login(
    user_auth: web::Data<AccountService>,
    login_request: Json<LoginRequest>,
) -> HttpResponse {
    match user_auth.login(login_request.into_inner()) {
        Ok(token_string) => HttpResponse::Ok().json(TokenResponse::bearer(token_string)),
        Err(e) => HttpResponse::InternalServerError().body(e),
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
    let auth_header = req.headers().get(constants::AUTHORIZATION_HEADER);
    match auth_header {
        Some(header) => match user_auth.logout(header.to_str().unwrap().to_string()) {
            Ok(_) => HttpResponse::Ok().json(json!({"message": "User logged out successfully"})),
            Err(e) => HttpResponse::InternalServerError().body(e),
        },
        None => HttpResponse::BadRequest().body("Authorization header not found"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Arc::new(establish_connection_pool(&database_url));
    let auth_service = AccountService::new(pool);
    let users_scope = web::scope("/users")
        .app_data(web::Data::new(auth_service.clone()))
        .service(login)
        .service(logout);

    cfg.service(users_scope);
}
