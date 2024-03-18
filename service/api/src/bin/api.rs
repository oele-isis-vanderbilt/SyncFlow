use actix_web::{web, App, HttpResponse, HttpServer};
use api::apidoc::init_api_doc;
use api::auth_middleware;
use api::livekit_handlers::init_routes as lk_init_routes;
use api::login_handlers::init_routes as login_init_routes;
use application::livekit::room::RoomService;
use application::mmla::mmla_service::MMLAService;
use application::mmla::user_actions::UserActions;
use application::users::account_service::AccountService;
use env_logger;
use infrastructure::establish_connection_pool;
use shared::deployment_config::DeploymentConfig;
use shared::response_models::Response;
use shared::utils::load_env;
use std::env;
use std::sync::Arc;
use application::livekit::egress::EgressService;

pub async fn not_found() -> actix_web::Result<HttpResponse> {
    let response = Response {
        status: 404,
        message: "Resource not found".to_string(),
    };

    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();
    env::set_var("RUST_LOG", "actix_web=debug,api=debug,application=debug,infrastructure=debug,shared=debug");
    env_logger::init();
    let config = DeploymentConfig::load();

    let app_host = config.app_host.clone();
    let app_port = config.app_port.clone();
    let num_workers = config.num_actix_workers.clone();

    let server_addr = format!("{}:{}", app_host, app_port);
    let database_url = config.database_url.clone();
    let pool = Arc::new(establish_connection_pool(&database_url));
    let auth_service = AccountService::new(pool.clone(), config.clone());
    let room_service = RoomService::new(
        config.livekit_server_url.clone(),
        config.livekit_api_key.clone(),
        config.livekit_api_secret.clone(),
    );
    let egress_service = EgressService::new(
        config.livekit_server_url.clone(),
        config.livekit_api_key.clone(),
        config.livekit_api_secret.clone(),
    );
    let user_actions = UserActions::new(pool.clone());
    let mmla_service = MMLAService::new(room_service, egress_service, user_actions);

    HttpServer::new(move || {
        App::new()
            .wrap(auth_middleware::Authentication) // Comment this line if you want to integrate with yew-address-book-frontend
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(mmla_service.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(lk_init_routes)
            .configure(login_init_routes)
            .configure(init_api_doc)
    })
    .workers(num_workers)
    .bind(server_addr)?
    .run()
    .await
}
