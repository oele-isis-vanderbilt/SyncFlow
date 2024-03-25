use crate::http_client::ClientError;
use crate::http_client::JSONResult;
use reqwest::blocking::Client;
use shared::response_models::Response;
use shared::user_models::{
    ApiKeyRequest, ApiKeyResponse, ApiKeyResponseWithoutSecret, LoginRequest, TokenResponse,
};

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
            username_or_email: username.to_string(),
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

    pub fn generate_api_keys(&self, token: &str, comment: &str) -> JSONResult<ApiKeyResponse> {
        let url = format!("{}/users/api-key", self.base_url);
        let api_key_request = ApiKeyRequest {
            comment: comment.to_string(),
        };
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&api_key_request)
            .send();

        match response {
            Ok(r) => {
                if !r.status().is_success() {
                    return Err(ClientError::from(Response {
                        status: r.status().as_u16(),
                        message: r.text().unwrap(),
                    }));
                } else {
                    let api_key_response = r.json::<ApiKeyResponse>();
                    api_key_response.map_err(|e| ClientError::from(e))
                }
            }
            Err(e) => Err(ClientError::from(e)),
        }
    }

    pub fn list_api_keys(&self, token: &str) -> JSONResult<Vec<ApiKeyResponseWithoutSecret>> {
        let url = format!("{}/users/api-keys", self.base_url);
        let response = self
            .client
            .get(&url)
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
                    let api_key_response = r.json::<Vec<ApiKeyResponseWithoutSecret>>();
                    api_key_response.map_err(|e| ClientError::from(e))
                }
            }
            Err(e) => Err(ClientError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::deployment_config::DeploymentConfig;

    fn setup_login_client() -> Option<LoginClient> {
        let config = DeploymentConfig::load();
        let app_host = config.app_host;
        let app_port = config.app_port;
        let base_url: String = format!("http://{}:{}", app_host, app_port);

        let login_client = LoginClient::new(&base_url);
        Some(login_client)
    }

    #[test]
    fn test_login_logout() {
        let login_client = setup_login_client().unwrap();
        let config = DeploymentConfig::load();
        let username = config.test_user.unwrap();
        let password = config.test_password.unwrap();

        let login_result = login_client.login(&username, &password);
        assert!(login_result.is_ok());
        let auth_token = login_result.unwrap().token;

        let logout_result = login_client.logout(&auth_token);
        assert!(logout_result.is_ok());
    }

    #[test]
    fn test_generate_api_keys() {
        let login_client = setup_login_client().unwrap();
        let config = DeploymentConfig::load();
        let username = config.test_user.unwrap();
        let password = config.test_password.unwrap();

        let login_result = login_client.login(&username, &password);
        assert!(login_result.is_ok());
        let auth_token = login_result.unwrap().token;

        let comment = "Test API Key";
        let api_key_result = login_client.generate_api_keys(&auth_token, comment);
        assert!(api_key_result.is_ok());
        let response_api_key = api_key_result.unwrap().key;

        let list_api_keys_result = login_client.list_api_keys(&auth_token);
        assert!(list_api_keys_result.is_ok());
        let api_keys = list_api_keys_result.unwrap();
        let api_key = api_keys.iter().find(|k| k.key == response_api_key);
        assert!(api_key.is_some());
    }
}
