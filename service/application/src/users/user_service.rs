use shared::user_models::LoginRequest;
use infrastructure::{establish_connection_pool, DbPool};
use domain::models::User;
use bcrypt::{verify};
use diesel::prelude::*;
use std::sync::Arc;

pub struct UserAuth {
    pool: Arc<DbPool>
}

fn verify_passwd(password: &str, hash: &str) -> bool {
    let password_match = verify(password, hash);
    password_match.unwrap_or(false)
}

impl UserAuth {
    pub fn new(pool: Arc<DbPool>) -> Self {
        UserAuth { pool }
    }


    pub fn login(&self, request: LoginRequest) -> Result<bool, String> {
        use domain::schema::users::dsl::*;

        let (uname, passwd) = (request.username, request.password);
        let mut conn = self.pool.get().unwrap();

        let user = users
            .filter(username.eq(uname))
            .first::<User>(&mut conn);

        user.map(|u| verify_passwd(&passwd, &u.password))
            .map_err(|_e| "User not found".to_string())
    }

    pub fn logout() {
        unimplemented!()
    }
}



