use crate::utils::load_env;
use envious;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DeploymentConfig {
    pub app_host: String,
    pub app_port: u16,
    pub num_actix_workers: usize,
    pub jwt_secret: String,
    pub database_url: String,
    pub encryption_key: String,
    pub jwt_expiration: usize,
    pub jwt_refresh_expiration: usize,

    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,

    pub root_user: Option<RootUser>,

    pub rabbitmq_config: RabbitMQConfig,

    /// Test configuration
    pub login_token: Option<String>,
    pub test_user: Option<String>,
    pub test_password: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RootUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum StorageConfig {
    S3(S3Config),
    Local(LocalConfig),
}

#[derive(Deserialize, Debug, Clone)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LocalConfig {
    pub recording_root_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RabbitMQConfig {
    pub host: String,
    pub port: u16,
    pub root_username: String,
    pub root_password: String,
    pub exchange_name: String,
    pub queue_name: String,
}

impl DeploymentConfig {
    pub fn load() -> Self {
        load_env();
        match envious::Config::default().build_from_env::<DeploymentConfig>() {
            Ok(config) => config,
            Err(e) => panic!("Failed to load deployment config: {}", e),
        }
    }

    pub fn load_from_file(filepath: String) -> Self {
        dotenvy::from_path(filepath).expect("Failed to load deployment config");
        match envious::Config::default().build_from_env::<DeploymentConfig>() {
            Ok(config) => config,
            Err(e) => panic!("Failed to load deployment config: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let config = DeploymentConfig::load();
        assert!(!config.app_host.is_empty());
        assert!(config.app_port > 0);
        assert!(!config.jwt_secret.is_empty());
        assert!(!config.database_url.is_empty());

        assert!(config.login_token.is_some());
    }
}
