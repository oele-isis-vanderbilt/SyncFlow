#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use diesel::QueryDsl;
    use infrastructure::establish_connection_pool;
    use std::env;

    use crate::models::{Role, User, UserEgressAction};

    #[test]
    fn test_establish_connection_pool() {
        let env_var = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection_pool(&env_var);
        let _conn = pool.get().expect("Failed to get connection");
    }

    #[test]
    fn test_load_user() {
        use crate::schema::users::dsl::*;
        let env_var = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection_pool(&env_var);
        let mut conn = pool.get().expect("Failed to get connection");
        let user = users.filter(id.eq(1)).first::<User>(&mut conn);
        assert!(user.is_ok());
        assert_eq!(user.unwrap().role, Role::ADMIN);
    }

    #[test]
    fn test_user_egress_actions() {
        use crate::schema::egress_actions::dsl::*;
        let env_var = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection_pool(&env_var);
        let mut conn = pool.get().expect("Failed to get connection");
        let actions = egress_actions
            .filter(user_id.eq(1))
            .load::<UserEgressAction>(&mut conn);
        assert!(actions.is_ok());
    }
}
