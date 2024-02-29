use crate::livekit::room::{CreateRoomRequest, RoomService};
use crate::models::{RoomCreationResult, TokenRequest, TokenResponse};
use crate::{livekit, Response};
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse};
use utoipa::OpenApi;

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
    let token = livekit::token::create_token(&token_request);
    match token {
        Ok(t) => HttpResponse::Ok().json(TokenResponse::new(t, token_request.identity.clone())),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
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

#[utoipa::path(
    get,
    path = "/livekit/list-rooms",
    responses(
        (status = 200, description = "Listed Rooms Successfully", body = Vec<RoomCreationResult>),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/list-rooms")]
pub async fn list_rooms(room_service: web::Data<RoomService>) -> HttpResponse {
    let rooms = room_service.list_rooms(None).await;
    match rooms {
        Ok(rooms) => HttpResponse::Ok().json(
            rooms
                .into_iter()
                .map(move |r| RoomCreationResult::from(r))
                .collect::<Vec<RoomCreationResult>>(),
        ),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Add these routes as config
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(RoomService::new()));
    cfg.service(generate_token);
    cfg.service(create_room);
    cfg.service(delete_room);
    cfg.service(list_rooms);
}
