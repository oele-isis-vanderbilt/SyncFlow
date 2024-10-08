use std::future::{ready, Ready};

use actix_web::http::Method;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use application::users::account_service::AccountService;
use application::users::tokens_manager::UserInfo;
use futures_util::future::LocalBoxFuture;
use shared::constants;
use shared::response_models::Response;

pub struct Ownership;

fn extract_project_id(req: &ServiceRequest) -> Option<String> {
    let path = req.path();
    path.trim_start_matches("/projects/")
        .split("/")
        .next()
        .map(|s| s.to_string())
}

#[derive(Debug)]
enum OwnershipStatus {
    Ignored,
    ApplicationError(HttpResponse),
    Authorized,
}

impl<S, B> Transform<S, ServiceRequest> for Ownership
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = OwnershipMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OwnershipMiddleware { service }))
    }
}

pub struct OwnershipMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for OwnershipMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let status = if Method::OPTIONS == *req.method()
            || constants::IGNORE_PROJECT_OWNERSHIP_ROUTES
                .iter()
                .any(|ignore_route| req.path().starts_with(ignore_route))
        {
            OwnershipStatus::Ignored
        } else if let Some(user_info) = req.extensions().get::<UserInfo>() {
            let account_service = req.app_data::<Data<AccountService>>();

            account_service
                .map(|svc| {
                    let project_id = extract_project_id(&req);
                    if let Some(project_id) = project_id {
                        let user_id = user_info.user_id;
                        svc.get_project(user_id, &project_id)
                            .map(|_| OwnershipStatus::Authorized)
                            .unwrap_or_else(|e: application::users::user::UserError| {
                                OwnershipStatus::ApplicationError(Response::from(e).into())
                            })
                    } else {
                        OwnershipStatus::ApplicationError(
                            HttpResponse::BadRequest().body("Project ID Not Found"),
                        )
                    }
                })
                .unwrap_or_else(|| {
                    OwnershipStatus::ApplicationError(
                        HttpResponse::Unauthorized().body("Account Service Not Found"),
                    )
                })
        } else {
            OwnershipStatus::ApplicationError(
                HttpResponse::Unauthorized().body("User Info Not Found"),
            )
        };

        match status {
            OwnershipStatus::Ignored | OwnershipStatus::Authorized => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_body(|_, _body| EitherBody::left(_body)))
                })
            }
            OwnershipStatus::ApplicationError(resp) => {
                let (request, _pl) = req.into_parts();
                Box::pin(async { Ok(ServiceResponse::new(request, resp.map_into_right_body())) })
            }
        }
    }
}
