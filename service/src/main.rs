use crate::livekit::room::{CreateRoomRequest, RoomOptions, RoomService};
use crate::models::{RoomCreationResult, TokenRequest, TokenResponse, VideoGrantsWrapper};
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;
use std::env;

mod livekit;
mod models;
mod schema;
mod utils;

mod repository;

use crate::utils::load_env;
use utoipa::{OpenApi, ToSchema};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema)]
pub struct Response {
    pub status: String,
    pub message: String,
}

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

    #[derive(OpenApi)]
    #[openapi(
        paths(
            livekit::routes::generate_token,
            livekit::routes::create_room,
            livekit::routes::delete_room,
            livekit::routes::list_rooms
        ),
       components(
            schemas(Response, TokenRequest, TokenResponse, VideoGrantsWrapper, CreateRoomRequest, RoomOptions, RoomCreationResult)
       ),
        tags(
            (name = "LiveKit MMLA API", description = "Room and Token Management Endpoints")
        ),
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    let app_port: u16 = match std::env::var("PORT") {
        Ok(val) => val.parse().unwrap_or(8081),
        Err(_) => 8081,
    };

    let app_host = std::env::var("HOST").expect("HOST must be set");

    let server_addr = format!("{}:{}", app_host, app_port);

    HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(not_found))
            .service(web::scope("/livekit").configure(livekit::routes::config))
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(server_addr)?
    .run()
    .await
}
