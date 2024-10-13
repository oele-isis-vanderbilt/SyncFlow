use actix_web::web::ServiceConfig;
use shared::livekit_models::{
    CreateRoomRequest, LivekitRoom, RoomOptions, TokenRequest, TokenResponse, VideoGrantsWrapper,
};
use shared::response_models::Response;
use shared::user_models::{
    LoginRequest, ProjectInfo, ProjectRequest, RefreshTokenRequest, SignUpRequest,
};

use shared::project_models::ProjectsSummary;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub fn init_api_doc(config: &mut ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            crate::login_handlers::login,
            crate::login_handlers::logout,
            crate::login_handlers::refresh_login_token,
            crate::login_handlers::signup,
            crate::login_handlers::me,
            crate::project_handlers::create_project,
            crate::project_handlers::get_project,
            crate::project_handlers::list_projects,
            crate::project_handlers::delete_project,
            // crate::project_handlers::summarize_projects
        ),
        components(
            schemas(Response, LoginRequest, RefreshTokenRequest, SignUpRequest, ProjectRequest, ProjectInfo, ProjectsSummary,
            TokenRequest, TokenResponse, VideoGrantsWrapper, CreateRoomRequest, RoomOptions, LivekitRoom)
        ),
        tags(
            (name = "SyncFlow API", description = "SyncFlowAPI for LiveKit and User Management"),
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
