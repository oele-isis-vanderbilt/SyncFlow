use actix_web::{web, App, HttpResponse, HttpServer};
use api::apidoc::init_api_doc;
use api::login_handlers::init_routes as login_init_routes;
use api::oauth_handlers::init_github_oauth_routes;
use api::project_handlers::init_routes as project_init_routes;
use api::{auth_middleware, rmq_handlers};

use application::project::devices::device_service;
use application::project::session_service::SessionService;
use application::rmq::auth::RMQAuthService;
use application::rmq::session_notifier::SessionNotifier;
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

    let session_service = SessionService::new(&config.encryption_key, pool.clone());
    let device_service = device_service::DeviceService::new(&config, pool.clone());
    let rmq_auth_service = RMQAuthService::new(auth_service.clone(), config.clone(), pool.clone());
    let session_notifier_service = SessionNotifier::create(config.rabbitmq_config.clone())
        .await
        .expect("Failed to create session notifier");

    session_notifier_service
        .initialize()
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to initialize session notifier: {}", e);
        });

    info!("Session notifier initialized with queue");

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(auth_middleware::Authentication) // Comment this line if you want to integrate with yew-address-book-frontend
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(login_init_routes)
            .configure(init_api_doc)
            .configure(|cfg| {
                project_init_routes(
                    cfg,
                    web::Data::new(session_service.clone()),
                    web::Data::new(device_service.clone()),
                    web::Data::new(session_notifier_service.clone()),
                )
            })
            .configure(|cfg| {
                rmq_handlers::init_routes(cfg, web::Data::new(rmq_auth_service.clone()))
            });

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
