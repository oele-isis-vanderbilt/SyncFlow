use super::super::livekit::room::RoomService;
use crate::{
    livekit::egress::EgressService,
    users::secret::{decrypt_string, encrypt_string, key_secret_pair, SecretError},
};
use diesel::{prelude::*, PgConnection};
use domain::models::{
    NewProject, NewProjectAPIKey, Project, ProjectAPIKey, ProjectDevice, ProjectSession,
    ProjectSessionStatus, StorageType,
};
use livekit_api::services::ServiceError;
use shared::{
    deployment_config::{S3Config, StorageConfig},
    project_models::{ProjectSummary, ProjectsSummary},
    user_models::ProjectRequest,
};

use super::super::s3::storage_service::StorageService;

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

    #[error("Livekit Error: {0}")]
    LivekitError(#[from] ServiceError),
}

pub(crate) trait Encryptable<T = Self> {
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
        let access_key = decrypt_string(&self.access_key, key)?;
        let secret_key = decrypt_string(&self.secret_key, key)?;
        let livekit_server_api_key = decrypt_string(&self.livekit_server_api_key, key)?;
        let livekit_server_api_secret = decrypt_string(&self.livekit_server_api_secret, key)?;

        self.access_key = access_key;
        self.secret_key = secret_key;
        self.livekit_server_api_key = livekit_server_api_key;
        self.livekit_server_api_secret = livekit_server_api_secret;
        Ok(())
    }
}

impl Encryptable for Project {
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
        let access_key = decrypt_string(&self.access_key, key)?;
        let secret_key = decrypt_string(&self.secret_key, key)?;
        let livekit_server_api_key = decrypt_string(&self.livekit_server_api_key, key)?;
        let livekit_server_api_secret = decrypt_string(&self.livekit_server_api_secret, key)?;

        self.access_key = access_key;
        self.secret_key = secret_key;
        self.livekit_server_api_key = livekit_server_api_key;
        self.livekit_server_api_secret = livekit_server_api_secret;
        Ok(())
    }
}

impl Encryptable for NewProjectAPIKey {
    fn encrypt(&mut self, key: &str) -> Result<(), SecretError> {
        let api_secret = encrypt_string(&self.api_secret, key)?;

        self.api_secret = api_secret;

        Ok(())
    }

    fn decrypt(&mut self, key: &str) -> Result<(), SecretError> {
        let api_secret = decrypt_string(&self.api_secret, key)?;

        self.api_secret = api_secret;

        Ok(())
    }
}

impl Encryptable for ProjectAPIKey {
    fn encrypt(&mut self, key: &str) -> Result<(), SecretError> {
        let api_secret = encrypt_string(&self.api_key, key)?;

        self.api_secret = api_secret;

        Ok(())
    }

    fn decrypt(&mut self, key: &str) -> Result<(), SecretError> {
        let api_secret = decrypt_string(&self.api_secret, key)?;

        self.api_secret = api_secret;

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
            ProjectError::LivekitError(e) => shared::response_models::Response {
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

pub(crate) fn get_project_by_id(
    proj_id: &str,
    conn: &mut PgConnection,
) -> Result<Project, ProjectError> {
    use domain::schema::syncflow::projects::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;

    let project = projects
        .filter(id.eq(proj_uuid))
        .first::<Project>(conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => {
                ProjectError::ProjectNotFoundError("Project not found".to_string())
            }
            _ => ProjectError::DatabaseError(err),
        })?;

    Ok(project)
}

pub(crate) fn summarize_projects(
    uid: i32,
    conn: &mut PgConnection,
) -> Result<ProjectsSummary, ProjectError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let user_projects = list_projects(uid, conn)?;
    let total_projects = user_projects.len();

    let num_sessions = project_sessions
        .filter(project_id.eq_any(user_projects.iter().map(|p| p.id).collect::<Vec<Uuid>>()))
        .count()
        .get_result::<i64>(conn)?;

    let num_active_sessions = project_sessions
        .filter(
            project_id
                .eq_any(user_projects.iter().map(|p| p.id).collect::<Vec<Uuid>>())
                .and(status.eq(ProjectSessionStatus::Started)),
        )
        .count()
        .get_result::<i64>(conn)?;

    Ok(ProjectsSummary {
        num_projects: total_projects as u32,
        num_active_sessions: num_active_sessions as u32,
        num_sessions: num_sessions as u32,
    })
}

pub async fn summarize_project(
    proj_id: &str,
    conn: &mut PgConnection,
    encryption_secret: &str,
) -> Result<ProjectSummary, ProjectError> {
    use domain::schema::syncflow::project_sessions::dsl::*;

    let mut project = get_project_by_id(proj_id, conn)?;
    project.decrypt(encryption_secret)?;
    let room_service: RoomService = (&project).into();
    let egress_service: EgressService = (&project).into();

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;

    let sessions = project_sessions
        .filter(project_id.eq(proj_uuid))
        .load::<ProjectSession>(conn)?;

    let num_sessions = sessions.len();

    let active_sessions = sessions
        .iter()
        .filter(|s| s.status == ProjectSessionStatus::Started)
        .cloned()
        .collect::<Vec<ProjectSession>>();

    let num_active_sessions = active_sessions.len();

    let mut total_participants = 0;
    let mut total_egresses = 0;

    for session in active_sessions {
        let participants = room_service
            .list_participants(&session.livekit_room_name)
            .await?;
        total_participants += participants.len();
        let egresses = egress_service
            .list_egresses(&session.livekit_room_name)
            .await?;
        total_egresses += egresses.len();
    }

    Ok(ProjectSummary {
        num_sessions: num_sessions as u32,
        num_active_sessions: num_active_sessions as u32,
        num_participants: total_participants as u32,
        num_recordings: total_egresses as u32,
    })
}

impl From<&Project> for RoomService {
    fn from(project: &Project) -> Self {
        let config = StorageConfig::S3(S3Config {
            bucket: project.bucket_name.clone(),
            region: project.region.clone().unwrap_or_default(),
            access_key: project.access_key.clone(),
            secret_key: project.secret_key.clone(),
            endpoint: project.endpoint.clone(),
        });

        RoomService::new(
            project.livekit_server_url.clone(),
            project.livekit_server_api_key.clone(),
            project.livekit_server_api_secret.clone(),
            project.get_recording_root(),
            config,
        )
    }
}

impl From<&Project> for EgressService {
    fn from(value: &Project) -> Self {
        let config = StorageConfig::S3(S3Config {
            bucket: value.bucket_name.clone(),
            region: value.region.clone().unwrap_or_default(),
            access_key: value.access_key.clone(),
            secret_key: value.secret_key.clone(),
            endpoint: value.endpoint.clone(),
        });

        EgressService::new(
            value.livekit_server_url.clone(),
            value.livekit_server_api_key.clone(),
            value.livekit_server_api_secret.clone(),
            config,
        )
    }
}

impl From<&Project> for StorageService {
    fn from(value: &Project) -> Self {
        let config = S3Config {
            bucket: value.bucket_name.clone(),
            region: value.region.clone().unwrap_or_default(),
            access_key: value.access_key.clone(),
            secret_key: value.secret_key.clone(),
            endpoint: value.endpoint.clone(),
        };

        StorageService::new(&config)
    }
}

pub fn create_api_key(
    uid: i32,
    proj_id: &str,
    key_comments: Option<String>,
    encryption_secret: &str,
    conn: &mut PgConnection,
) -> Result<ProjectAPIKey, ProjectError> {
    use domain::schema::syncflow::project_api_keys::dsl::*;

    let key_pair = key_secret_pair();

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;

    let mut new_key = NewProjectAPIKey {
        project_id: proj_uuid,
        user_id: uid,
        api_key: key_pair.key.clone(),
        api_secret: key_pair.secret.clone(),
        comments: key_comments.clone(),
    };

    new_key.encrypt(encryption_secret)?;

    diesel::insert_into(project_api_keys)
        .values(&new_key)
        .get_result::<ProjectAPIKey>(conn)
        .map_err(ProjectError::DatabaseError)
}

pub fn list_all_api_keys(
    uid: i32,
    proj_id: &str,
    conn: &mut PgConnection,
) -> Result<Vec<ProjectAPIKey>, ProjectError> {
    use domain::schema::syncflow::project_api_keys::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;
    let keys = project_api_keys
        .filter(user_id.eq(uid).and(project_id.eq(proj_uuid)))
        .load::<ProjectAPIKey>(conn)?;

    Ok(keys)
}

pub fn delete_api_key(
    uid: i32,
    proj_id: &str,
    key_id: i32,
    conn: &mut PgConnection,
) -> Result<ProjectAPIKey, ProjectError> {
    use domain::schema::syncflow::project_api_keys::dsl::*;

    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;

    let key = diesel::delete(
        project_api_keys.filter(
            user_id
                .eq(uid)
                .and(project_id.eq(proj_uuid))
                .and(id.eq(key_id)),
        ),
    )
    .get_result::<ProjectAPIKey>(conn)?;

    Ok(key)
}

pub fn fetch_api_key_by_key(
    key: &str,
    proj_id: &str,
    conn: &mut PgConnection,
) -> Result<ProjectAPIKey, ProjectError> {
    use domain::schema::syncflow::project_api_keys::dsl::*;
    let proj_uuid = Uuid::parse_str(proj_id)
        .map_err(|_| ProjectError::ConfigurationError("Invalid project id".to_string()))?;

    let key = project_api_keys
        .filter(api_key.eq(key).and(project_id.eq(proj_uuid)))
        .first::<ProjectAPIKey>(conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => {
                ProjectError::ProjectNotFoundError("API Key not found".to_string())
            }
            _ => ProjectError::DatabaseError(err),
        })?;

    Ok(key)
}

pub fn project_contains_device_group(
    proj_id: &str,
    device_group_name: &str,
    conn: &mut PgConnection,
) -> Result<Vec<ProjectDevice>, ProjectError> {
    use domain::schema::syncflow::project_devices::dsl::*;

    let project = get_project_by_id(proj_id, conn)?;

    let devices = project_devices
        .filter(
            project_id
                .eq(project.id)
                .and(device_group.eq(device_group_name)),
        )
        .load::<ProjectDevice>(conn)?;

    Ok(devices)
}
