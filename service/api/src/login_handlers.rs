use actix_web::web::Json;
use actix_web::{post, web, HttpRequest, HttpResponse};
use application::users::account_service::AccountService;
use shared::constants;
use shared::response_models::Response;
use shared::user_models::{LoginRequest, TokenResponse};

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
    match req.headers().get(constants::AUTHORIZATION_HEADER) {
        Some(header) => {
            let token = header.to_str().unwrap().split(" ").collect::<Vec<&str>>()[1];
            match user_auth.logout(token) {
                Ok(_) => HttpResponse::Ok().json(Response {
                    status: "200".to_string(),
                    message: "User logged out successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().body(e),
            }
        }
        None => HttpResponse::InternalServerError().body("Invalid token"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let users_scope = web::scope("/users").service(login).service(logout);
    cfg.service(users_scope);
}
