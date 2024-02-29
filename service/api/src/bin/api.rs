use actix_web::{web, App, HttpResponse, HttpServer};
use api::apidoc::init_api_doc;
use api::livekit_handlers::{init_routes as lk_init_routes};
use api::login_handlers::{init_routes as login_init_routes};
use shared::response_models::Response;
use shared::utils::load_env;
use std::env;
use env_logger;

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

    let server_addr = format!("{}:{}", app_host, app_port);

    HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(not_found))
            .configure(lk_init_routes)
            .configure(login_init_routes)
            .configure(init_api_doc)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(server_addr)?
    .run()
    .await
}
