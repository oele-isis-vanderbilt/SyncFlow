use std::sync::Arc;

use infrastructure::DbPool;

use crate::project::session::{self, SessionError};
use shared::project_models::{NewSessionRequest, NewSessionResponse};

pub struct SessionService {
    encryption_key: String,
    pool: Arc<DbPool>,
}

impl SessionService {
    pub fn new(encryption_key: &str, pool: Arc<DbPool>) -> Self {
        SessionService {
            encryption_key: encryption_key.to_string(),
            pool,
        }
    }

    pub async fn create_session(&self, project_id: &str, session: NewSessionRequest) -> Result<NewSessionResponse, SessionError> {
        let new_session = session::create_session(project_id, session, &self.encryption_key, &mut self.pool.get().unwrap()).await;

        new_session.map(|session| session.into())

    }

    pub fn get_sessions(&self, project_id: &str) {}

    pub fn get_session(&self, project_id: &str, session_id: i32) {}

    pub fn get_session_token(&self, project_id: &str, session_id: i32) {}

    pub fn delete_session(&self, project_id: &str, session_id: i32) {}
}
