use crate::{
    helpers::{error_response, json_ok_response},
    ownership_middleware,
};
use actix_web::{
    delete, get, middleware, post,
    web::{self, ReqData},
    HttpResponse,
};
use application::users::{account_service::AccountService, tokens_manager::UserInfo};
use shared::user_models::ProjectRequest;

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
    user_data: ReqData<UserInfo>,
    account_service: web::Data<AccountService>,
) -> HttpResponse {
    account_service
        .get_projects(user_data.into_inner().user_id)
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
    user_data: ReqData<UserInfo>,
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
    user_data: ReqData<UserInfo>,
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
    user_data: ReqData<UserInfo>,
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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let projects_scope = web::scope("/projects")
        .wrap(ownership_middleware::Ownership)
        .service(create_project)
        .service(list_projects)
        .service(get_project)
        .service(delete_project);

    cfg.service(projects_scope);
}
