use actix_web::{get, web, HttpResponse};

use application::rmq::auth::{
    RMQAuthQuery, RMQAuthResourcePathQuery, RMQAuthService, RMQAuthTopicQuery, RMQAuthVhostQuery,
};

#[get("/auth/user")]
async fn authorize_user(
    auth_query: web::Query<RMQAuthQuery>,
    auth_service: web::Data<RMQAuthService>,
) -> HttpResponse {
    let is_user_valid = auth_service.authorize(&auth_query.into_inner());
    if is_user_valid {
        HttpResponse::Ok().body("allow administrator")
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/auth/vhost")]
async fn authorize_vhost(
    vhost_query: web::Query<RMQAuthVhostQuery>,
    auth_service: web::Data<RMQAuthService>,
) -> HttpResponse {
    let is_user_valid = auth_service.authorize_vhost(&vhost_query.into_inner());
    if is_user_valid {
        HttpResponse::Ok().body("allow")
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/auth/resource")]
async fn authorize_resource_path(
    resource_path_query: web::Query<RMQAuthResourcePathQuery>,
    auth_service: web::Data<RMQAuthService>,
) -> HttpResponse {
    let is_user_valid = auth_service.authorize_resource_path(&resource_path_query.into_inner());
    if is_user_valid {
        HttpResponse::Ok().body("allow")
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/auth/topic")]
async fn authorize_topic(
    topic_query: web::Query<RMQAuthTopicQuery>,
    auth_service: web::Data<RMQAuthService>,
) -> HttpResponse {
    let is_user_valid = auth_service.authorize_topic(&topic_query.into_inner());
    if is_user_valid {
        HttpResponse::Ok().body("allow")
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig, auth_service: web::Data<RMQAuthService>) {
    let rmq_scope = web::scope("/rmq")
        .app_data(auth_service.clone())
        .service(authorize_user)
        .service(authorize_vhost)
        .service(authorize_resource_path)
        .service(authorize_topic);

    cfg.service(rmq_scope);
}
