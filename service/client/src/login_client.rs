use crate::http_client::ClientError;
use crate::http_client::JSONResult;
use reqwest::blocking::Client;
use shared::response_models::Response;
use shared::user_models::{LoginRequest, TokenResponse};

pub struct LoginClient {
    client: Client,
    base_url: String,
}

impl LoginClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        LoginClient {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub fn login(&self, username: &str, password: &str) -> JSONResult<TokenResponse> {
        let url = format!("{}/users/login", self.base_url);
        let login_request = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&login_request)
            .send();

        match response {
            Ok(r) => {
                if !r.status().is_success() {
                    return Err(ClientError::from(Response {
                        status: r.status().as_u16(),
                        message: r.text().unwrap(),
                    }));
                }
                let token_response = r.json::<TokenResponse>();
                token_response.map_err(|e| ClientError::from(e))
            }
            Err(e) => Err(ClientError::from(e)),
        }
    }

    pub fn logout(&self, token: &str) -> JSONResult<Response> {
        let url = format!("{}/users/logout", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send();

        match response {
            Ok(r) => {
                if !r.status().is_success() {
                    return Err(ClientError::from(Response {
                        status: r.status().as_u16(),
                        message: r.text().unwrap(),
                    }));
                } else {
                    let logout_response = r.json::<Response>();
                    logout_response.map_err(|e| ClientError::from(e))
                }
            }
            Err(e) => Err(ClientError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    fn setup_login_client() -> Option<LoginClient> {
        dotenv().ok();
        let app_host: String = env::var("APP_HOST").unwrap_or("localhost".to_string());
        let app_port: String = env::var("APP_PORT").unwrap_or("8081".to_string());
        let base_url: String = format!("http://{}:{}", app_host, app_port);

        let login_client = LoginClient::new(&base_url);
        Some(login_client)
    }

    #[test]
    fn test_login_logout() {
        let login_client = setup_login_client().unwrap();
        let username = env::var("TEST_USER").unwrap_or("test_user".to_string());
        let password = env::var("TEST_PASSWORD").unwrap_or("test_password".to_string());

        let login_result = login_client.login(&username, &password);
        assert!(login_result.is_ok());
        let auth_token = login_result.unwrap().token;

        let logout_result = login_client.logout(&auth_token);
        assert!(logout_result.is_ok());
    }
}
