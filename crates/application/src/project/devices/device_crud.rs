use crate::rmq::session_notifier::SessionNotifierError;
use diesel::{prelude::*, PgConnection};
use domain::models::{NewProjectDevice, ProjectDevice};
use shared::device_models::DeviceRegisterRequest;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Database Error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Configuration Error: {0}")]
    ConfigurationError(#[from] uuid::Error),

    #[error("Device not found")]
    NotFound(String),

    #[error("Session Notifier Error: {0}")]
    SessionNotifierError(#[from] SessionNotifierError),
}

impl From<DeviceError> for shared::response_models::Response {
    fn from(error: DeviceError) -> Self {
        match error {
            DeviceError::DatabaseError(e) => match e {
                diesel::result::Error::NotFound => shared::response_models::Response {
                    status: 404,
                    message: e.to_string(),
                },
                _ => shared::response_models::Response {
                    status: 500,
                    message: e.to_string(),
                },
            },
            DeviceError::ConfigurationError(e) => shared::response_models::Response {
                status: 400,
                message: e.to_string(),
            },
            DeviceError::NotFound(e) => shared::response_models::Response {
                status: 404,
                message: e,
            },
            DeviceError::SessionNotifierError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
        }
    }
}

pub fn register_device(
    proj_id: &str,
    uid: i32,
    registration_request: &DeviceRegisterRequest,
    conn: &mut PgConnection,
) -> Result<ProjectDevice, DeviceError> {
    use domain::schema::syncflow::project_devices::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)?;

    let new_project_device = NewProjectDevice {
        project_id: proj_uuid,
        registered_by: uid,
        comments: registration_request.comments.clone(),
        device_name: registration_request.name.clone(),
        device_group: registration_request.group.clone(),
    };
    let project_device = diesel::insert_into(project_devices)
        .values(new_project_device)
        .get_result(conn)?;

    Ok(project_device)
}

pub fn list_devices(
    proj_id: &str,
    conn: &mut PgConnection,
) -> Result<Vec<ProjectDevice>, DeviceError> {
    use domain::schema::syncflow::project_devices::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)?;

    let devices = project_devices
        .filter(project_id.eq(proj_uuid))
        .load::<ProjectDevice>(conn)?;

    Ok(devices)
}

pub fn get_device(
    proj_id: &str,
    device_id: &str,
    conn: &mut PgConnection,
) -> Result<ProjectDevice, DeviceError> {
    use domain::schema::syncflow::project_devices::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)?;

    let device_uuid = Uuid::parse_str(device_id)?;

    let device = project_devices
        .filter(project_id.eq(proj_uuid).and(id.eq(device_uuid)))
        .first::<ProjectDevice>(conn)?;

    Ok(device)
}

pub fn delete_device(
    proj_id: &str,
    device_id: &str,
    conn: &mut PgConnection,
) -> Result<ProjectDevice, DeviceError> {
    use domain::schema::syncflow::project_devices::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)?;

    let device_uuid = Uuid::parse_str(device_id)?;

    let device =
        diesel::delete(project_devices.filter(project_id.eq(proj_uuid).and(id.eq(device_uuid))))
            .get_result::<ProjectDevice>(conn)?;

    Ok(device)
}
