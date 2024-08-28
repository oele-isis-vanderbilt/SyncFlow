use std::sync::Arc;

use infrastructure::DbPool;

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

    // pub fn create_session(&self, project_id, session: NewSessionRequest)  {}
    // pub fn get_session(&self, session_id: i32)  {}
    // pub fn delete_session(&self, session_id: i32)  {}
    // pub fn update_session(&self, session_id: i32, session: NewSessionRequest)  {}
}
