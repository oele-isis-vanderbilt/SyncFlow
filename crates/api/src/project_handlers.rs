use crate::{
    helpers::{error_response, json_ok_response},
    ownership_middleware,
};
use actix_web::{
    delete, get, post,
    web::{self, ReqData},
    HttpResponse,
};
use application::{
    project::{devices::device_service::DeviceService, session_service::SessionService},
    rmq::session_notifier::SessionNotifier,
    users::{account_service::AccountService, tokens_manager::TokenInfo},
};
use shared::{
    device_models::DeviceRegisterRequest,
    livekit_models::TokenRequest,
    project_models::NewSessionRequest,
    user_models::{ApiKeyRequest, ProjectRequest},
};

#[utoipa::path(
    get,
    path = "/projects/list",
    responses(
        (status = 200, description = "List of Projects", body = Vec<ProjectInfo>),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    )
)]
#[get("/list")]
async fn list_projects(
    user_data: ReqData<TokenInfo>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    account_service
        .get_projects(user_data.into_inner().user_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    get,
    path = "/projects/summarize",
    responses(
        (status = 200, description = "Summary of projects", body = ProjectsSummary),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    )
)]
#[get("/summarize")]
async fn summarize_projects(
    user_data: ReqData<TokenInfo>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    account_service
        .summarize_projects(user_data.into_inner().user_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    get,
    path = "/projects/{project_id}",
    responses(
        (status = 200, description = "Project Details", body = ProjectInfo),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    ),
    params(
        ("project_id", description = "The ID of the project to get details")
    )
)]
#[get("/{project_id}")]
async fn get_project(
    user_data: ReqData<TokenInfo>,
    project_id: web::Path<String>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    account_service
        .get_project(user_data.into_inner().user_id, &project_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    delete,
    path = "/projects/{project_id}",
    responses(
        (status = 200, description = "Project Details", body = ProjectInfo),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    ),
    params(
        ("project_id", description = "The ID of the project to get details")
    )
)]
#[delete("/{project_id}")]
async fn delete_project(
    user_data: ReqData<TokenInfo>,
    project_id: web::Path<String>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    account_service
        .delete_project(user_data.into_inner().user_id, &project_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    post,
    path = "/projects/create",
    request_body = ProjectRequest,
    responses(
        (status = 200, description = "Project Created Successfully", body = ProjectInfo),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    )
)]
#[post("/create")]
async fn create_project(
    user_data: ReqData<TokenInfo>,
    project_request: web::Json<ProjectRequest>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    account_service
        .create_project(
            user_data.into_inner().user_id,
            &project_request.into_inner(),
        )
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    get,
    path = "/projects/{project_id}/summarize",
    responses(
        (status = 200, description = "Project Details", body = ProjectSummary),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    ),
    params(
        ("project_id", description = "The ID of the project to get details")
    )
)]
#[get("/{project_id}/summarize")]
async fn summarize_project(
    project_id: web::Path<String>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    account_service
        .summarize_project(&project_id)
        .await
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    post,
    path = "/projects/{project_id}/create-session",
    request_body = NewSessionRequest,
    responses(
        (status = 200, description = "Session Created Successfully", body = ProjectSession),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    ),
    params(
        ("project_id", description = "The ID of the project to create session")
    )
)]
#[post("/{project_id}/create-session")]
async fn create_session(
    project_id: web::Path<String>,
    session: web::Json<NewSessionRequest>,
    session_service: web::Data<SessionService>,
    notifier_service: web::Data<SessionNotifier>,
) -> HttpResponse {
    session_service
        .create_session(
            &project_id,
            &session.into_inner(),
            &notifier_service.into_inner(),
        )
        .await
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[utoipa::path(
    get,
    path = "/projects/{project_id}/sessions",
    responses(
        (status = 200, description = "List of Sessions", body = Vec<ProjectSessionResponse>),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal Server Error/DatabaseError")
    ),
    params(
        ("project_id", description = "The ID of the project to get sessions")
    )
)]
#[get("/{project_id}/sessions")]
async fn get_sessions(
    project_id: web::Path<String>,
    session_service: web::Data<SessionService>,
) -> HttpResponse {
    session_service
        .get_sessions(&project_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[get("/{project_id}/sessions/{session_id}")]
async fn get_session(
    path: web::Path<(String, String)>,
    session_service: web::Data<SessionService>,
) -> HttpResponse {
    let (project_id, session_id) = path.into_inner();
    session_service
        .get_session(&project_id, &session_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[delete("/{project_id}/sessions/{session_id}")]
async fn delete_session(
    path: web::Path<(String, String)>,
    session_service: web::Data<SessionService>,
) -> HttpResponse {
    let (project_id, session_id) = path.into_inner();
    session_service
        .delete_session(&project_id, &session_id)
        .await
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[get("/{project_id}/sessions/{session_id}/participants")]
async fn get_participants(
    path: web::Path<(String, String)>,
    session_service: web::Data<SessionService>,
) -> HttpResponse {
    let (project_id, session_id) = path.into_inner();
    session_service
        .get_participants(&project_id, &session_id)
        .await
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[get("/{project_id}/sessions/{session_id}/livekit-session-info")]
async fn get_livekit_session_info(
    path: web::Path<(String, String)>,
    session_service: web::Data<SessionService>,
) -> HttpResponse {
    let (project_id, session_id) = path.into_inner();
    session_service
        .livekit_session_info(&project_id, &session_id)
        .await
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[post("/{project_id}/sessions/{session_id}/token")]
pub async fn generate_session_token(
    path: web::Path<(String, String)>,
    token_request: web::Json<TokenRequest>,
    session_service: web::Data<SessionService>,
) -> HttpResponse {
    let (project_id, session_id) = path.into_inner();
    session_service
        .get_session_token(&project_id, &session_id, &token_request.into_inner())
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[post("/{project_id}/sessions/{session_id}/stop")]
async fn stop_session(
    path: web::Path<(String, String)>,
    session_service: web::Data<SessionService>,
) -> HttpResponse {
    let (project_id, session_id) = path.into_inner();
    session_service
        .stop_session(&project_id, &session_id)
        .await
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[post("/{project_id}/settings/create-api-key")]
async fn create_api_key(
    project_id: web::Path<String>,
    user_info: ReqData<TokenInfo>,
    account_service: web::Data<AccountService>,
    request: web::Json<ApiKeyRequest>,
) -> HttpResponse {
    let user_id = user_info.into_inner().user_id;
    account_service
        .create_project_api_key(user_id, &project_id, &request)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[get("{project_id}/settings/api-keys")]
async fn get_all_api_keys(
    project_id: web::Path<String>,
    user_info: ReqData<TokenInfo>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    let user_id = user_info.into_inner().user_id;
    account_service
        .list_project_api_keys(user_id, &project_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[delete("{project_id}/settings/api-keys/{api_key_id}")]
async fn delete_api_key(
    path: web::Path<(String, i32)>,
    user_info: ReqData<TokenInfo>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    let (project_id, api_key_id) = path.into_inner();
    let user_id = user_info.into_inner().user_id;
    account_service
        .delete_project_api_key(user_id, &project_id, api_key_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[get("{project_id}/devices")]
async fn list_devices(
    project_id: web::Path<String>,
    device_service: web::Data<DeviceService>,
) -> HttpResponse {
    device_service
        .list_devices(&project_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[get("{project_id}/devices/{device_id}")]
async fn get_device(
    path: web::Path<(String, String)>,
    device_service: web::Data<DeviceService>,
) -> HttpResponse {
    let (project_id, device_id) = path.into_inner();
    device_service
        .get_device(&project_id, &device_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[post("{project_id}/devices/register")]
async fn register_device(
    project_id: web::Path<String>,
    user_info: ReqData<TokenInfo>,
    request: web::Json<DeviceRegisterRequest>,
    device_service: web::Data<DeviceService>,
    notifier_service: web::Data<SessionNotifier>,
) -> HttpResponse {
    device_service
        .register_device(
            &project_id,
            user_info.into_inner().user_id,
            &request.into_inner(),
            &notifier_service.into_inner(),
        )
        .await
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

#[delete("{project_id}/devices/{device_id}")]
async fn delete_device(
    path: web::Path<(String, String)>,
    device_service: web::Data<DeviceService>,
) -> HttpResponse {
    let (project_id, device_id) = path.into_inner();
    device_service
        .delete_device(&project_id, &device_id)
        .map(json_ok_response)
        .unwrap_or_else(error_response)
}

pub fn init_routes(
    cfg: &mut web::ServiceConfig,
    session_service: web::Data<SessionService>,
    device_service: web::Data<DeviceService>,
    notifier_service: web::Data<SessionNotifier>,
) {
    let projects_scope = web::scope("/projects")
        .wrap(ownership_middleware::Ownership)
        .app_data(session_service.clone())
        .app_data(notifier_service.clone())
        .service(create_project)
        .service(list_projects)
        .service(summarize_projects)
        .service(get_project)
        .service(delete_project)
        .service(summarize_project)
        .service(create_session)
        .service(delete_session)
        .service(generate_session_token)
        .service(get_participants)
        .service(get_sessions)
        .service(get_session)
        .service(stop_session)
        .service(get_livekit_session_info)
        .service(create_api_key)
        .service(get_all_api_keys)
        .service(delete_api_key)
        .app_data(device_service.clone())
        .service(list_devices)
        .service(get_device)
        .service(register_device)
        .service(delete_device);

    cfg.service(projects_scope);
}
