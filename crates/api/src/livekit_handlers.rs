use actix_web::web::{Json, ReqData};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use application::mmla::mmla_service::MMLAService;
use application::users::tokens_manager::UserInfo;
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
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let token_result = mmla_service
                .generate_token(
                    user_info.into_inner().user_id,
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
            HttpResponse::Unauthorized().body("Unauthorized")
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
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();

            let req_json = room_create_request.into_inner();
            println!("{:?}", req_json);
            let create_room_result = mmla_service.create_room(user_info.user_id, req_json).await;

            match create_room_result {
                Ok(room) => HttpResponse::Ok().json(room),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            HttpResponse::Unauthorized().body("Unauthorized")
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
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let delete_room_result = mmla_service
                .delete_room(user_info.user_id, room_name.to_owned())
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
            HttpResponse::Unauthorized().body("Unauthorized")
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
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let list_rooms_result = mmla_service.list_rooms(user_info.user_id).await;

            match list_rooms_result {
                Ok(rooms) => HttpResponse::Ok().json(rooms),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            HttpResponse::Unauthorized().body("Unauthorized")
        }
    }
}

#[utoipa::path(
    get,
    path = "/livekit/list-participants/{room_name}",
    responses(
        (status = 200, description = "List of participants"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("room_name", description = "The name of the room to get participants")
    )
)]
#[get("/list-participants/{room_name}")]
pub async fn list_participants(
    mmla_service: web::Data<MMLAService>,
    room_name: web::Path<String>,
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let list_participants_result = mmla_service
                .list_participants(user_info.user_id, &room_name)
                .await;

            match list_participants_result {
                Ok(participants) => HttpResponse::Ok().json(participants),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            HttpResponse::Unauthorized().body("Unauthorized")
        }
    }
}

#[utoipa::path(
    get,
    path = "/livekit/list-egresses/{room_name}",
    responses(
        (status = 200, description = "List of egresses"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("room_name", description = "The name of the room to get egresses")
    )
)]
#[get("/list-egresses/{room_name}")]
pub async fn list_egresses(
    mmla_service: web::Data<MMLAService>,
    room_name: web::Path<String>,
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let list_egresses_result = mmla_service
                .list_egresses(user_info.user_id, &room_name)
                .await;

            match list_egresses_result {
                Ok(egresses) => HttpResponse::Ok().json(egresses),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            HttpResponse::Unauthorized().body("Unauthorized")
        }
    }
}

#[utoipa::path(
    post,
    path = "/livekit/begin-track-egress/{room_name}/{track_sid}",
    responses(
        (status = 200, description = "Track egress started"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/begin-track-egress/{room_name}/{track_sid}")]
pub async fn begin_track_egress(
    mmla_service: web::Data<MMLAService>,
    params: web::Path<(String, String)>,
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    let (room_name, track_sid) = params.into_inner();
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let begin_egress_result = mmla_service
                .record_track(user_info.user_id, &room_name, &track_sid)
                .await;

            match begin_egress_result {
                Ok(egress_result) => HttpResponse::Ok().json(egress_result),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            HttpResponse::Unauthorized().body("Unauthorized")
        }
    }
}

#[utoipa::path(
    post,
    path = "/livekit/stop-recording/{room_name}/{egress_id}",
    responses(
        (status = 200, description = "Recording stopped"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/stop-recording/{room_name}/{egress_id}")]
pub async fn stop_recording(
    mmla_service: web::Data<MMLAService>,
    params: web::Path<(String, String)>,
    user_data: Option<ReqData<UserInfo>>,
) -> HttpResponse {
    let (room_name, track_sid) = params.into_inner();
    match user_data {
        Some(user_info) => {
            let user_info = user_info.into_inner();
            let stop_recording_result = mmla_service
                .stop_recording(user_info.user_id, &room_name, &track_sid)
                .await;

            match stop_recording_result {
                Ok(egress_result) => HttpResponse::Ok().json(egress_result),
                Err(err) => {
                    let response: Response = err.into();
                    response.into()
                }
            }
        }
        None => {
            HttpResponse::Unauthorized().body("Unauthorized")
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let livekit_scope = web::scope("/livekit")
        .service(healthcheck)
        .service(generate_token)
        .service(create_room)
        .service(delete_room)
        .service(list_rooms)
        .service(list_participants)
        .service(list_egresses)
        .service(begin_track_egress)
        .service(stop_recording);

    cfg.service(livekit_scope);
}
