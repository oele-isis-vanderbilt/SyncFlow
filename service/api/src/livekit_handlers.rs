use actix_web::web::Json;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use application::livekit::room::RoomService;
use application::livekit::token::create_token;
use shared::livekit_models::{CreateRoomRequest, RoomCreationResult, TokenRequest, TokenResponse};
use shared::response_models::Response;
use shared::utils::ping_livekit;

#[utoipa::path(
    get,
    path = "/livekit/health",
    responses((status = 200, description = "Health check successful", body = Response),)
)]
#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    let response = ping_livekit().await;

    if response {
        HttpResponse::Ok().json(Response {
            status: 200,
            message: "Livekit server is healthy".to_string(),
        })
    } else {
        HttpResponse::InternalServerError().body("Livekit server ping failed")
    }
}

#[utoipa::path(
    post,
    path = "/livekit/token",
    request_body = TokenRequest,
    responses(
        (status = 200, description = "Added a livekit room join token", body = TokenResponse),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/token")]
pub async fn generate_token(token_request: Json<TokenRequest>) -> HttpResponse {
    let token = create_token(&token_request).map_err(|e| Response {
        status: 500,
        message: e.to_string(),
    });
    match token {
        Ok(t) => HttpResponse::Ok().json(TokenResponse::new(t, token_request.identity.clone())),
        Err(e) => e.into(),
    }
}

#[utoipa::path(
    post,
    path = "/livekit/create-room",
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
    path = "/livekit/delete-room/{room_name}",
    responses(
        (status = 200, description = "Room created successfully", body = Response),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("room_name", description = "The name of the room to delete")
    )
)]
#[delete("/delete-room/{room_name}")]
pub async fn delete_room(
    room_service: web::Data<RoomService>,
    room_name: web::Path<String>,
) -> HttpResponse {
    let delete_room_result = room_service.delete_room(&room_name).await;

    match delete_room_result {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: 200,
            message: format!(" Room {} deleted successfully", room_name),
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/livekit/list-rooms",
    responses(
        (status = 200, description = "List of rooms", body = Vec<RoomCreationResult>),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/list-rooms")]
pub async fn list_rooms(room_service: web::Data<RoomService>) -> HttpResponse {
    let list_rooms_result = room_service.list_rooms(None).await;

    match list_rooms_result {
        Ok(rooms) => {
            let room_results: Vec<RoomCreationResult> =
                rooms.into_iter().map(RoomCreationResult::from).collect();
            HttpResponse::Ok().json(room_results)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let app_data = web::Data::new(RoomService::new());
    let livekit_scope = web::scope("/livekit")
        .app_data(app_data.clone())
        .service(healthcheck)
        .service(generate_token)
        .service(create_room)
        .service(delete_room)
        .service(list_rooms);

    cfg.service(livekit_scope);
}
