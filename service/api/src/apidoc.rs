use actix_web::web::ServiceConfig;
use shared::livekit_models::{
    CreateRoomRequest, RoomCreationResult, RoomOptions, TokenRequest, TokenResponse,
    VideoGrantsWrapper,
};
use shared::user_models::LoginRequest;
use shared::response_models::Response;
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
            crate::login_handlers::login,
        ),
        components(
            schemas(Response, LoginRequest,
            TokenRequest, TokenResponse, VideoGrantsWrapper, CreateRoomRequest, RoomOptions, RoomCreationResult)
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
