use actix_web::web::ServiceConfig;
use shared::livekit_models::{
    CreateRoomRequest, LivekitRoom, RoomOptions, TokenRequest, TokenResponse, VideoGrantsWrapper,
};
use shared::response_models::Response;
use shared::user_models::{LoginRequest, RefreshTokenRequest};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub fn init_api_doc(config: &mut ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            crate::livekit_handlers::healthcheck,
            crate::livekit_handlers::generate_token,
            crate::livekit_handlers::create_room,
            crate::livekit_handlers::delete_room,
            crate::livekit_handlers::list_rooms,
            crate::livekit_handlers::list_participants,
            crate::livekit_handlers::list_egresses,
            crate::livekit_handlers::begin_track_egress,
            crate::livekit_handlers::stop_recording,
            crate::login_handlers::login,
            crate::login_handlers::logout,
            crate::login_handlers::refresh_login_token
        ),
        components(
            schemas(Response, LoginRequest, RefreshTokenRequest,
            TokenRequest, TokenResponse, VideoGrantsWrapper, CreateRoomRequest, RoomOptions, LivekitRoom)
        ),
        tags(
            (name = "LiveKit MMLA API", description = "Room and Token Management Endpoints")
        ),
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    config
        .service(Redoc::with_url("/redoc", openapi.clone()))
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        )
        .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"));
}
