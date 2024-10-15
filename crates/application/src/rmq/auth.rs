use crate::users::account_service::AccountService;
use shared::deployment_config::DeploymentConfig;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RMQAuthQuery {
    pub username: String,
    pub password: String,
}

pub struct RMQAuthService {
    account_service: AccountService,
    deployment_config: DeploymentConfig,
}

impl RMQAuthService {
    pub fn new(account_service: AccountService, deployment_config: DeploymentConfig) -> Self {
        Self {
            account_service,
            deployment_config,
        }
    }

    pub fn authorize(&self, auth_query: &RMQAuthQuery) -> bool {
        if auth_query.username == self.deployment_config.rabbitmq_config.root_username
            && auth_query.password == self.deployment_config.rabbitmq_config.root_password
        {
            true
        } else {
            self.account_service
                .verify_token(&auth_query.password)
                .is_ok()
        }
    }
}

impl Clone for RMQAuthService {
    fn clone(&self) -> Self {
        Self {
            account_service: self.account_service.clone(),
            deployment_config: self.deployment_config.clone(),
        }
    }
}
