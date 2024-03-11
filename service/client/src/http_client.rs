use reqwest::blocking::{Client, Response};
use serde::Serialize;
use shared::response_models::Response as SharedResponse;

#[derive(Debug)]
pub enum ClientError {
    ReqwestError(reqwest::Error),
    HTTPError(SharedResponse),
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::ReqwestError(error)
    }
}

impl From<SharedResponse> for ClientError {
    fn from(error: SharedResponse) -> Self {
        ClientError::HTTPError(error)
    }
}

pub type JSONResult<T> = std::result::Result<T, ClientError>;

// ToDo: Use API Key and Secret based authentication
pub struct HTTPAuthTokenClient {
    base_url: String,
    token: String,
    client: Client,
}

impl HTTPAuthTokenClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let client = Client::new();
        let base_url = format!("{}", base_url);
        HTTPAuthTokenClient {
            base_url,
            token: token.to_string(),
            client,
        }
    }

    pub fn get(&self, path: &str) -> reqwest::Result<Response> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send();
        response
    }

    pub fn post<T: Serialize>(&self, path: &str, body: T) -> reqwest::Result<Response> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send();
        response
    }

    pub fn delete(&self, path: &str) -> reqwest::Result<Response> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send();

        response
    }

    // Allow dead code
    #[allow(dead_code)]
    pub fn put<T: Serialize>(&self, path: &str, body: T) -> reqwest::Result<Response> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send();
        response
    }

    pub fn map_response<T>(&self, response: reqwest::Result<Response>) -> JSONResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match response {
            Ok(response) => {
                let parsed = response.json::<T>();
                parsed.map_err(|e| ClientError::from(e))
            }
            Err(e) => Err(ClientError::from(e)),
        }
    }
}
