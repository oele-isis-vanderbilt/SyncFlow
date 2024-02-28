use crate::models::User;
use crate::utils::{get_conn_pool, DbPool};
use diesel::prelude::*;

pub trait IUserRepository {
    fn get_user_by_username(&self, username: &str) -> Result<User, String>;
    fn get_user_by_email(&self, email: &str) -> Result<User, String>;
}

pub struct UserRepository {
    pool: DbPool,
}

impl IUserRepository for UserRepository {
    fn get_user_by_username(&self, username: &str) -> Result<User, String> {
        use crate::schema::users::dsl::{username as uname, users};

        let mut conn = self.pool.get().unwrap();
        let user = users.filter(uname.eq(&username)).first::<User>(&mut conn);
        match user {
            Ok(u) => Ok(u),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_user_by_email(&self, email: &str) -> Result<User, String> {
        use crate::schema::users::dsl::{email as usr_email, users};

        let mut conn = self.pool.get().unwrap();
        let user = users.filter(usr_email.eq(&email)).first::<User>(&mut conn);
        match user {
            Ok(u) => Ok(u),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl UserRepository {
    pub fn new() -> Self {
        let pool = get_conn_pool();
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_env;

    #[test]
    fn test_get_user_by_username() {
        load_env();
        let repo = UserRepository::new();
        let user = repo.get_user_by_username("admin");
        assert!(user.is_ok());
        assert_eq!(user.unwrap().email, "admin@elp.org");
    }

    #[test]
    fn test_get_user_by_email() {
        load_env();
        let repo = UserRepository::new();
        let user = repo.get_user_by_email("admin@elp.org");

        assert!(user.is_ok());
        assert_eq!(user.unwrap().username, "admin");
    }
}
