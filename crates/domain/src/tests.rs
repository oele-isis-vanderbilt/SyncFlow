#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use diesel::QueryDsl;
    use infrastructure::establish_connection_pool;
    use std::env;

    use crate::models::User;

    #[test]
    fn test_establish_connection_pool() {
        let env_var = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection_pool(&env_var);
        let _conn = pool.get().expect("Failed to get connection");
    }

    #[test]
    fn test_load_user() {
        use crate::schema::syncflow::users::dsl::*;
        let env_var = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection_pool(&env_var);
        let mut conn = pool.get().expect("Failed to get connection");
        let user = users.filter(id.eq(1)).first::<User>(&mut conn);
        assert!(user.is_ok());
    }
}
