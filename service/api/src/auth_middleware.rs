use std::future::{ready, Ready};

use actix_web::http::Method;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpResponse,
};
use application::users::account_service::AccountService;
use application::users::token::{decode_token, verify_token};
use futures_util::future::LocalBoxFuture;
use log::{error, info};
use shared::constants;
use shared::response_models::Response;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
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
        let mut auth_success = false;

        // Options Should be Passed Freely
        if Method::OPTIONS == *req.method() {
            auth_success = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    auth_success = true;
                    break;
                }
            }
        }

        if !auth_success {
            if let Some(account_service) = req.app_data::<Data<AccountService>>() {
                info!("Connecting to database");
                let pool = account_service.get_pool();

                let mut conn = pool.get().unwrap();
                if let Some(auth_header) = req.headers().get(constants::AUTHORIZATION_HEADER) {
                    info!("Parsing authorization header...");
                    if let Ok(auth_string) = auth_header.to_str() {
                        if auth_string.starts_with("bearer") || auth_string.starts_with("Bearer") {
                            info!("Parsing Token...");
                            let token = auth_string[6..auth_string.len()].trim();

                            if let Ok(token_data) = decode_token(token.to_string()) {
                                info!("Decoding Token...");
                                if verify_token(&token_data, &mut conn).is_ok() {
                                    info!("Valid Token");
                                    auth_success = true;
                                }
                            } else {
                                error!("Invalid Token");
                            }
                        }
                    }
                }
            }
        }

        if !auth_success {
            let (request, _pl) = req.into_parts();

            let response = HttpResponse::Unauthorized()
                .json(Response {
                    message: constants::MESSAGE_INVALID_TOKEN.to_string(),
                    status: 401,
                })
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(req);

        Box::pin(async move {
            let res = res.await?;
            Ok(res.map_body(|_, _body| EitherBody::left(_body)))
        })
    }
}
