use actix_web::{
    post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use application::users::{account_service::AccountService, oauth::github::GithubUser};
use shared::response_models::Response;
use shared::user_models::TokenResponse;

#[post("/github/login")]
async fn login_with_github(
    req: HttpRequest,
    account_service: web::Data<AccountService>,
    user: Json<GithubUser>,
) -> HttpResponse {
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return HttpResponse::Unauthorized().body("Authorization Header not found");
    }
    let auth_header = auth_header.unwrap();
    let user_data = user.into_inner();
    if let Ok(auth_string) = auth_header.to_str() {
        let github_token = auth_string[6..auth_string.len()].trim();
        let token_result = account_service
            .login_with_github(github_token, &user_data)
            .await;

        match token_result {
            Ok((access_token, refresh_token)) => {
                return HttpResponse::Ok().json(TokenResponse::bearer(access_token, refresh_token));
            }
            Err(e) => {
                let response: Response = e.into();
                return response.into();
            }
        }
    } else {
        return HttpResponse::Unauthorized().body("Invalid Authorization Header");
    }
}

pub fn init_github_oauth_routes(cfg: &mut web::ServiceConfig) {
    // Deployment Config load
    let livekit_scope = web::scope("/oauth").service(login_with_github);
    cfg.service(livekit_scope);
}
