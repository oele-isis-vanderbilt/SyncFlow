use actix_web::web::{Json, ReqData};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use application::mmla::mmla_service::MMLAService;
use application::users::token::UserTokenType;
use shared::deployment_config::DeploymentConfig;
use shared::livekit_models::{CreateRoomRequest, TokenRequest};
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
pub async fn generate_token(
    token_request: Json<TokenRequest>,
    mmla_service: web::Data<MMLAService>,
    deployment_config: web::Data<DeploymentConfig>,
    token_data: Option<ReqData<UserTokenType>>,
) -> HttpResponse {
    match token_data {
        Some(token) => {
            let token_result = mmla_service
                .generate_token(
                    token.into_inner().claims.user_id,
                    token_request.into_inner(),
                    deployment_config.livekit_api_key.clone(),
                    deployment_config.livekit_api_secret.clone(),
                )
                .await;

            match token_result {
                Ok(token) => HttpResponse::Ok().json(token),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
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
    mmla_service: web::Data<MMLAService>,
    room_create_request: Json<CreateRoomRequest>,
    token_data: Option<ReqData<UserTokenType>>,
) -> HttpResponse {
    match token_data {
        Some(token) => {
            let token_inner = token.into_inner();

            let req_json = room_create_request.into_inner();
            println!("{:?}", req_json);
            let create_room_result = mmla_service
                .create_room(token_inner.claims.user_id, req_json)
                .await;

            match create_room_result {
                Ok(room) => HttpResponse::Ok().json(room),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
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
    mmla_service: web::Data<MMLAService>,
    room_name: web::Path<String>,
    token_data: Option<ReqData<UserTokenType>>,
) -> HttpResponse {
    match token_data {
        Some(token) => {
            let token_inner = token.into_inner();
            let delete_room_result = mmla_service
                .delete_room(token_inner.claims.user_id, room_name.to_owned())
                .await;

            match delete_room_result {
                Ok(success) => success.into(),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
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
pub async fn list_rooms(
    mmla_service: web::Data<MMLAService>,
    token_data: Option<ReqData<UserTokenType>>,
) -> HttpResponse {
    match token_data {
        Some(token) => {
            let token_inner = token.into_inner();
            let list_rooms_result = mmla_service.list_rooms(token_inner.claims.user_id).await;

            match list_rooms_result {
                Ok(rooms) => HttpResponse::Ok().json(rooms),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let livekit_scope = web::scope("/livekit")
        .service(healthcheck)
        .service(generate_token)
        .service(create_room)
        .service(delete_room)
        .service(list_rooms);

    cfg.service(livekit_scope);
}
