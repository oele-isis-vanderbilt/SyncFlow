use crate::livekit::room::{CreateRoomRequest, RoomOptions, RoomService};
use crate::models::{RoomCreationResult, TokenRequest, TokenResponse, VideoGrantsWrapper};
use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::env;

mod auth;
mod livekit;
mod models;
mod schema;
mod utils;

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

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Health check successful", body = Response),)
)]
#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    let response = Response {
        status: "200".to_string(),
        message: "Everything is working".to_string(),
    };

    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    post,
    path = "/token",
    request_body = TokenRequest,
    responses(
        (status = 200, description = "Added a livekit room join token", body = TokenResponse),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/token")]
pub async fn generate_token(token_request: Json<TokenRequest>) -> HttpResponse {
    let token = livekit::token::create_token(&token_request);
    match token {
        Ok(t) => HttpResponse::Ok().json(TokenResponse::new(t, token_request.identity.clone())),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/create-room",
    request_body = CreateRoomRequest,
    responses(
        (status = 200, description = "Room created successfully", body = RoomCreationResult),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/create-room")]
pub async fn create_room(
    room_service: web::Data<RoomService>,
    room_create_request: Json<CreateRoomRequest>,
) -> HttpResponse {
    let create_room_result = room_service
        .create_room(
            &room_create_request.name,
            room_create_request.options.clone(),
        )
        .await;

    match create_room_result {
        Ok(room) => HttpResponse::Ok().json(RoomCreationResult::from(room)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/delete-room/{room_name}",
    responses(
        (status = 200, description = "Room created successfully", body = Response),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("room_name", description = "The name of the room to delete")
    )
)]
#[post("/delete-room/{room_name}")]
pub async fn delete_room(
    room_service: web::Data<RoomService>,
    room_name: web::Path<String>,
) -> HttpResponse {
    let delete_room_result = room_service.delete_room(&room_name).await;

    match delete_room_result {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "200".to_string(),
            message: format!(" Room {} deleted successfully", room_name),
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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
            healthcheck,
            generate_token,
            create_room,
            delete_room,
        ),
       components(
            schemas(Response, TokenRequest, TokenResponse, VideoGrantsWrapper, CreateRoomRequest, RoomOptions, RoomCreationResult)
       ),
        tags(
            (name = "LiveKit MMLA API", description = "Room and Token Management Endpoints")
        ),
    )]
    struct ApiDoc;

    let room_service = RoomService::new();

    let app_data = web::Data::new(room_service);

    let openapi = ApiDoc::openapi();

    let app_port: u16 = match std::env::var("PORT") {
        Ok(val) => val.parse().unwrap_or(8081),
        Err(_) => 8081,
    };

    let app_host = std::env::var("HOST").expect("HOST must be set");

    let server_addr = format!("{}:{}", app_host, app_port);

    HttpServer::new(move || {
        App::new()
            .service(healthcheck)
            .app_data(app_data.clone())
            .default_service(web::route().to(not_found))
            .service(create_room)
            .service(delete_room)
            .service(generate_token)
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
