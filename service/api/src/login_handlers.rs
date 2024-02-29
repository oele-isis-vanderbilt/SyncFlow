use shared::user_models::LoginRequest;
use actix_web::{web, HttpResponse, post};
use actix_web::web::Json;
use application::users::auth::UserAuth;


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
    user_auth: web::Data<UserAuth>,
    login_request: Json<LoginRequest>
) -> HttpResponse {
    match user_auth.login(login_request.into_inner()) {
        Ok(true) => HttpResponse::Ok().json(true),
        Ok(false) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(e)
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let app_data = web::Data::new(UserAuth::new());
    let users_scope = web::scope("/users")
        .app_data(app_data.clone())
        .service(login);

    cfg.service(users_scope);
}