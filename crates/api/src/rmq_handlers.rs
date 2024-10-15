use actix_web::{get, web, HttpResponse};

use application::rmq::auth::{RMQAuthQuery, RMQAuthService};

#[get("/auth/user")]
async fn authorize_user(
    auth_query: web::Query<RMQAuthQuery>,
    auth_service: web::Data<RMQAuthService>,
) -> HttpResponse {
    // HttpResponse::Ok().finish()
    let is_user_valid = auth_service.authorize(&auth_query.into_inner());
    if is_user_valid {
        HttpResponse::Ok().body("allow administrator")
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

// #[get("/auth/")]

pub fn init_routes(cfg: &mut web::ServiceConfig, auth_service: web::Data<RMQAuthService>) {
    let rmq_scope = web::scope("/rmq")
        .app_data(auth_service.clone())
        .service(authorize_user);

    cfg.service(rmq_scope);
}
