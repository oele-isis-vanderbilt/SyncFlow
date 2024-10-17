use std::sync::Arc;

use domain::models::ProjectDevice;
use infrastructure::DbPool;
use shared::deployment_config::DeploymentConfig;
use shared::device_models::{DeviceRegisterRequest, DeviceResponse};

use super::device_crud::{self, DeviceError};
use crate::rmq::session_notifier::SessionNotifier;

pub struct DeviceService {
    config: DeploymentConfig,
    pool: Arc<DbPool>,
}

impl DeviceService {
    pub fn new(config: &DeploymentConfig, pool: Arc<DbPool>) -> Self {
        Self {
            config: config.clone(),
            pool,
        }
    }

    pub async fn register_device(
        &self,
        project_id: &str,
        user_id: i32,
        registration_request: &DeviceRegisterRequest,
        notifier: &SessionNotifier,
    ) -> Result<DeviceResponse, DeviceError> {
        let device = device_crud::register_device(
            project_id,
            user_id,
            registration_request,
            &mut self.pool.get().unwrap(),
        )?;

        let routing_key = self.routing_key_for(&device);
        notifier.bind_routing_key(&routing_key).await?;

        Ok(self.device_to_response(&device))
    }

    pub fn list_devices(&self, project_id: &str) -> Result<Vec<DeviceResponse>, DeviceError> {
        let devices = device_crud::list_devices(project_id, &mut self.pool.get().unwrap())?;
        Ok(devices.iter().map(|d| self.device_to_response(d)).collect())
    }

    pub fn get_device(
        &self,
        project_id: &str,
        device_id: &str,
    ) -> Result<DeviceResponse, DeviceError> {
        let device = device_crud::get_device(project_id, device_id, &mut self.pool.get().unwrap())?;
        Ok(self.device_to_response(&device))
    }

    pub fn delete_device(
        &self,
        project_id: &str,
        device_id: &str,
    ) -> Result<DeviceResponse, DeviceError> {
        let device =
            device_crud::delete_device(project_id, device_id, &mut self.pool.get().unwrap())?;
        Ok(self.device_to_response(&device))
    }

    fn routing_key_for(&self, device: &ProjectDevice) -> String {
        format!("{}.{}", device.project_id, device.device_group)
    }

    fn device_to_response(&self, device: &ProjectDevice) -> DeviceResponse {
        let routing_key = self.routing_key_for(device);
        let exchange_name = &self.config.rabbitmq_config.exchange_name;
        device.into_device_response(&routing_key, exchange_name)
    }
}

impl Clone for DeviceService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            pool: self.pool.clone(),
        }
    }
}
