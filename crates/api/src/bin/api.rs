use actix_web::{web, App, HttpResponse, HttpServer};
use api::apidoc::init_api_doc;
use api::auth_middleware;
use api::livekit_handlers::init_routes as lk_init_routes;
use api::login_handlers::init_routes as login_init_routes;
use api::oauth_handlers::init_github_oauth_routes;
use api::project_handlers::init_routes as project_init_routes;
use application::livekit::egress::EgressService;
use application::livekit::room::RoomService;
use application::mmla::mmla_service::MMLAService;
use application::mmla::user_actions::UserActions;
use application::project::session_service::SessionService;
use application::users::account_service::AccountService;

use infrastructure::establish_connection_pool;
use log::info;
use shared::deployment_config::DeploymentConfig;
use shared::response_models::Response;
use shared::utils::load_env;
use std::env;
use std::sync::Arc;

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
    env::set_var(
        "RUST_LOG",
        "actix_web=debug,api=debug,application=debug,infrastructure=debug,shared=debug",
    );
    env_logger::init();
    let config = DeploymentConfig::load();

    let app_host = config.app_host.clone();
    let app_port = config.app_port;
    let num_workers = config.num_actix_workers;

    let server_addr = format!("{}:{}", app_host, app_port);
    let database_url = config.database_url.clone();
    let pool = Arc::new(establish_connection_pool(&database_url));
    let auth_service = AccountService::new(pool.clone(), config.clone());
    if config.root_user.is_some()
        && !auth_service.user_exists(config.root_user.as_ref().unwrap().username.as_str())
    {
        let root_user = config.root_user.as_ref().unwrap();
        let user = auth_service
            .create_user(&root_user.username, &root_user.email, &root_user.password)
            .unwrap_or_else(|e| panic!("Failed to create root user: {}", e));
        info!("Root user created: {:?}", user);
    }
    let room_service = RoomService::new(
        config.livekit_server_url.clone(),
        config.livekit_api_key.clone(),
        config.livekit_api_secret.clone(),
        config.storage_config.clone(),
    );
    let egress_service = EgressService::new(
        config.livekit_server_url.clone(),
        config.livekit_api_key.clone(),
        config.livekit_api_secret.clone(),
        config.storage_config.clone(),
    );
    let user_actions = UserActions::new(pool.clone());
    let mmla_service = MMLAService::new(room_service, egress_service, user_actions);
    let session_service = SessionService::new(&config.encryption_key, pool.clone());

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(auth_middleware::Authentication) // Comment this line if you want to integrate with yew-address-book-frontend
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(mmla_service.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(lk_init_routes)
            .configure(login_init_routes)
            .configure(init_api_doc)
            .configure(|cfg| project_init_routes(cfg, session_service.clone()));

        if config.github_client_id.is_some() && config.github_client_secret.is_some() {
            app = app.configure(init_github_oauth_routes);
        }

        app
    })
    .workers(num_workers)
    .bind(server_addr)?
    .run()
    .await
}
