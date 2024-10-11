use std::sync::Arc;

use infrastructure::DbPool;
use shared::deployment_config::DeploymentConfig;
use shared::device_models::{DeviceRegisterRequest, DeviceResponse};

use super::device_crud::{self, DeviceError};

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

    pub fn register_device(
        &self,
        project_id: &str,
        user_id: i32,
        registration_request: &DeviceRegisterRequest,
    ) -> Result<DeviceResponse, DeviceError> {
        let device = device_crud::register_device(
            project_id,
            user_id,
            registration_request,
            &mut self.pool.get().unwrap(),
        )?;
        Ok(device.into())
    }

    pub fn list_devices(&self, project_id: &str) -> Result<Vec<DeviceResponse>, DeviceError> {
        let devices = device_crud::list_devices(project_id, &mut self.pool.get().unwrap())?;
        Ok(devices.into_iter().map(Into::into).collect())
    }

    pub fn get_device(
        &self,
        project_id: &str,
        device_id: &str,
    ) -> Result<DeviceResponse, DeviceError> {
        let device = device_crud::get_device(project_id, device_id, &mut self.pool.get().unwrap())?;
        Ok(device.into())
    }

    pub fn delete_device(
        &self,
        project_id: &str,
        device_id: &str,
    ) -> Result<DeviceResponse, DeviceError> {
        let device =
            device_crud::delete_device(project_id, device_id, &mut self.pool.get().unwrap())?;
        Ok(device.into())
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
