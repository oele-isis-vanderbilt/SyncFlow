use crate::utils::load_env;
use envious;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DeploymentConfig {
    pub app_host: String,
    pub app_port: u16,
    pub num_actix_workers: usize,
    pub livekit_server_url: String,
    pub livekit_api_key: String,
    pub livekit_api_secret: String,
    pub jwt_secret: String,
    pub database_url: String,
    pub storage_config: StorageConfig,
    pub encryption_key: String,

    pub root_user: Option<RootUser>,
    pub text_egress_server_url: Option<String>,

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
        assert!(!config.livekit_server_url.is_empty());
        assert!(!config.livekit_api_key.is_empty());
        assert!(!config.livekit_api_secret.is_empty());
        assert!(!config.jwt_secret.is_empty());
        assert!(!config.database_url.is_empty());

        assert!(!config.login_token.is_none());
    }
}
