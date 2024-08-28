use crate::users::secret::{encrypt_string, SecretError};
use diesel::{prelude::*, PgConnection};
use domain::models::{NewProject, Project, StorageType};
use shared::user_models::ProjectRequest;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Project Not Found Error: {0}")]
    ProjectNotFoundError(String),

    #[error("Database Error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Storage Type Error: {0}")]
    ConfigurationError(String),

    #[error("Encryption Error: {0}")]
    EncryptionError(#[from] SecretError),
}

trait Encryptable<T = Self> {
    fn encrypt(&mut self, key: &str) -> Result<(), SecretError>;

    #[allow(unused)]
    fn decrypt(&mut self, key: &str) -> Result<(), SecretError>;
}

impl Encryptable for NewProject {
    fn encrypt(&mut self, key: &str) -> Result<(), SecretError> {
        let access_key = encrypt_string(&self.access_key, key)?;
        let secret_key = encrypt_string(&self.secret_key, key)?;
        let livekit_server_api_key = encrypt_string(&self.livekit_server_api_key, key)?;
        let livekit_server_api_secret = encrypt_string(&self.livekit_server_api_secret, key)?;

        self.access_key = access_key;
        self.secret_key = secret_key;
        self.livekit_server_api_key = livekit_server_api_key;
        self.livekit_server_api_secret = livekit_server_api_secret;

        Ok(())
    }

    fn decrypt(&mut self, key: &str) -> Result<(), SecretError> {
        let access_key = encrypt_string(&self.access_key, key)?;
        let secret_key = encrypt_string(&self.secret_key, key)?;
        let livekit_server_api_key = encrypt_string(&self.livekit_server_api_key, key)?;
        let livekit_server_api_secret = encrypt_string(&self.livekit_server_api_secret, key)?;

        self.access_key = access_key;
        self.secret_key = secret_key;
        self.livekit_server_api_key = livekit_server_api_key;
        self.livekit_server_api_secret = livekit_server_api_secret;
        Ok(())
    }
}

impl From<ProjectError> for shared::response_models::Response {
    fn from(val: ProjectError) -> Self {
        match val {
            ProjectError::ProjectNotFoundError(e) => shared::response_models::Response {
                status: 404,
                message: e,
            },
            ProjectError::DatabaseError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
            ProjectError::ConfigurationError(e) => shared::response_models::Response {
                status: 400,
                message: e,
            },
            ProjectError::EncryptionError(e) => shared::response_models::Response {
                status: 500,
                message: e.to_string(),
            },
        }
    }
}

pub fn create_project(
    uid: i32,
    new_project_request: &ProjectRequest,
    encryption_secret: &str,
    conn: &mut PgConnection,
) -> Result<Project, ProjectError> {
    use domain::schema::syncflow::projects::dsl::*;

    if new_project_request.storage_type != "s3" {
        return Err(ProjectError::ConfigurationError(
            "Storage type not found".to_string(),
        ));
    }

    let storage = StorageType::S3;

    let mut new_project = NewProject {
        user_id: uid,
        name: new_project_request.name.clone(),
        description: new_project_request.description.clone(),
        access_key: new_project_request.access_key.clone(),
        bucket_name: new_project_request.bucket_name.clone(),
        endpoint: new_project_request.endpoint.clone(),
        livekit_server_api_key: new_project_request.livekit_server_api_key.clone(),
        livekit_server_api_secret: new_project_request.livekit_server_api_secret.clone(),
        livekit_server_url: new_project_request.livekit_server_url.clone(),
        region: new_project_request.region.clone(),
        secret_key: new_project_request.secret_key.clone(),
        storage_type: storage,
    };

    new_project.encrypt(encryption_secret)?;

    let project = diesel::insert_into(projects)
        .values(&new_project)
        .get_result::<Project>(conn)?;

    Ok(project)
}

pub fn get_project(
    uid: i32,
    proj_id: &str,
    conn: &mut PgConnection,
) -> Result<Project, ProjectError> {
    use domain::schema::syncflow::projects::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;

    let project = projects
        .filter(user_id.eq(uid).and(id.eq(proj_uuid)))
        .first::<Project>(conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => {
                ProjectError::ProjectNotFoundError("Project not found".to_string())
            }
            _ => ProjectError::DatabaseError(err),
        })?;

    Ok(project)
}

pub fn list_projects(uid: i32, conn: &mut PgConnection) -> Result<Vec<Project>, ProjectError> {
    use domain::schema::syncflow::projects::dsl::*;

    let all_projects = projects.filter(user_id.eq(uid)).load::<Project>(conn)?;
    Ok(all_projects)
}

pub fn delete_project(
    uid: i32,
    proj_id: &str,
    conn: &mut PgConnection,
) -> Result<Project, ProjectError> {
    use domain::schema::syncflow::projects::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;

    let project = diesel::delete(projects.filter(user_id.eq(uid).and(id.eq(proj_uuid))))
        .get_result::<Project>(conn)?;

    Ok(project)
}

pub fn update_project(
    uid: i32,
    proj_id: Uuid,
    new_project_request: &ProjectRequest,
    encryption_secret: &str,
    conn: &mut PgConnection,
) -> Result<Project, ProjectError> {
    use domain::schema::syncflow::projects::dsl::*;

    projects
        .filter(user_id.eq(uid).and(id.eq(proj_id)))
        .first::<Project>(conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => {
                ProjectError::ProjectNotFoundError("Project not found".to_string())
            }
            _ => ProjectError::DatabaseError(err),
        })?;

    let storage = StorageType::S3;

    let mut updated_project = NewProject {
        user_id: uid,
        name: new_project_request.name.clone(),
        description: new_project_request.description.clone(),
        access_key: new_project_request.access_key.clone(),
        bucket_name: new_project_request.bucket_name.clone(),
        endpoint: new_project_request.endpoint.clone(),
        livekit_server_api_key: new_project_request.livekit_server_api_key.clone(),
        livekit_server_api_secret: new_project_request.livekit_server_api_secret.clone(),
        livekit_server_url: new_project_request.livekit_server_url.clone(),
        region: new_project_request.region.clone(),
        secret_key: new_project_request.secret_key.clone(),
        storage_type: storage,
    };

    updated_project.encrypt(encryption_secret)?;

    let project = diesel::update(projects.filter(user_id.eq(uid).and(id.eq(proj_id))))
        .set(&updated_project)
        .get_result::<Project>(conn)?;

    Ok(project)
}
