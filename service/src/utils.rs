use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

pub fn load_env() {
    match dotenv() {
        Ok(_) => {}
        Err(e) => {
            log::error!(
                "Failed to load .env file: {}, assuming variables are set",
                e
            );
        }
    };
}

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_conn_pool() -> DBPool {
    load_env();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
