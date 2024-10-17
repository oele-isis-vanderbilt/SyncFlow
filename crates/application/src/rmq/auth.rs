use crate::{
    project::{
        devices::device_crud::get_possible_routing_keys,
        project_crud::project_contains_device_group,
    },
    users::account_service::AccountService,
};
use infrastructure::DbPool;
use shared::deployment_config::DeploymentConfig;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RMQAuthQuery {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RMQAuthVhostQuery {
    pub username: String,
    pub vhost: String,
    pub ip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RMQAuthResourcePathQuery {
    pub username: String,
    pub vhost: String,
    pub resource: String,
    pub name: String,
    pub permission: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RMQAuthTopicQuery {
    pub username: String,
    pub vhost: String,
    pub resource: String,
    pub name: String,
    pub permission: String,
    pub routing_key: String,
}

pub struct RMQAuthService {
    account_service: AccountService,
    deployment_config: DeploymentConfig,
    pool: Arc<DbPool>,
}

impl RMQAuthService {
    pub fn new(
        account_service: AccountService,
        deployment_config: DeploymentConfig,
        pool: Arc<DbPool>,
    ) -> Self {
        Self {
            account_service,
            deployment_config,
            pool,
        }
    }

    pub fn authorize(&self, auth_query: &RMQAuthQuery) -> bool {
        let token_info = self.account_service.verify_token(&auth_query.username);
        token_info
            .map(|token| {
                let project_id = token.project_id;

                if project_id.is_none() {
                    false
                } else {
                    let project_id = project_id.unwrap();

                    project_contains_device_group(
                        &project_id,
                        &auth_query.password,
                        &mut self.pool.get().unwrap(),
                    )
                    .map(|_| true)
                    .unwrap_or(false)
                }
            })
            .unwrap_or(false)
    }

    pub fn authorize_vhost(&self, vhost_query: &RMQAuthVhostQuery) -> bool {
        let token_info = self.account_service.verify_token(&vhost_query.username);
        token_info
            .map(|_| vhost_query.vhost == self.deployment_config.rabbitmq_config.vhost_name)
            .unwrap_or(false)
    }

    pub fn authorize_resource_path(&self, resource_path_query: &RMQAuthResourcePathQuery) -> bool {
        if resource_path_query.vhost != self.deployment_config.rabbitmq_config.vhost_name {
            return false;
        }

        let token_info = self
            .account_service
            .verify_token(&resource_path_query.username);
        token_info
            .map(|token| {
                let project_id = token.project_id;
                if project_id.is_none() {
                    return false;
                }
                if resource_path_query.resource == "exchange"
                    && resource_path_query.name
                        == self.deployment_config.rabbitmq_config.exchange_name
                    && resource_path_query.permission == "read"
                {
                    return true;
                }

                if resource_path_query.resource == "queue"
                    && resource_path_query.name.starts_with("amq")
                    && resource_path_query.permission == "write"
                {
                    return true;
                }

                true
            })
            .unwrap_or(false)
    }

    pub fn authorize_topic(&self, topic_query: &RMQAuthTopicQuery) -> bool {
        if topic_query.vhost != self.deployment_config.rabbitmq_config.vhost_name {
            return false;
        }

        let token_info = self.account_service.verify_token(&topic_query.username);

        token_info
            .map(|token| {
                let project_id = token.project_id;
                if project_id.is_none() {
                    return false;
                }
                if topic_query.resource == "topic"
                    && topic_query.name == self.deployment_config.rabbitmq_config.exchange_name
                    && topic_query.permission == "read"
                {
                    let project_id = project_id.unwrap();
                    let all_routing_keys_result =
                        get_possible_routing_keys(&project_id, &mut self.pool.get().unwrap());

                    let access = all_routing_keys_result
                        .map(|routing_keys| routing_keys.contains(&topic_query.routing_key))
                        .unwrap_or(false);

                    return access;
                }
                true
            })
            .unwrap_or(false)
    }
}

impl Clone for RMQAuthService {
    fn clone(&self) -> Self {
        Self {
            account_service: self.account_service.clone(),
            deployment_config: self.deployment_config.clone(),
            pool: self.pool.clone(),
        }
    }
}
