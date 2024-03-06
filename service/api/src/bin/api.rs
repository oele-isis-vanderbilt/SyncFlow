use actix_web::{web, App, HttpResponse, HttpServer};
use api::apidoc::init_api_doc;
use api::auth_middleware;
use api::livekit_handlers::init_routes as lk_init_routes;
use api::login_handlers::init_routes as login_init_routes;
use application::users::account_service::AccountService;
use infrastructure::establish_connection_pool;
use shared::response_models::Response;
use shared::utils::load_env;
use std::env;
use std::sync::Arc;

pub async fn not_found() -> actix_web::Result<HttpResponse> {
    let response = Response {
        status: "404".to_string(),
        message: "Resource not found".to_string(),
    };

    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let num_workers = env::var("NUM_ACTIX_WORKERS")
        .unwrap_or_else(|_| "4".to_string())
        .parse::<usize>()
        .unwrap_or(4);

    let server_addr = format!("{}:{}", app_host, app_port);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Arc::new(establish_connection_pool(&database_url));
    let auth_service = AccountService::new(pool);

    HttpServer::new(move || {
        App::new()
            .wrap(auth_middleware::Authentication) // Comment this line if you want to integrate with yew-address-book-frontend
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(auth_service.clone()))
            .configure(lk_init_routes)
            .configure(login_init_routes)
            .configure(init_api_doc)
    })
    .workers(num_workers)
    .bind(server_addr)?
    .run()
    .await
}
