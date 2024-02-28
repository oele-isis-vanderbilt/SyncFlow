use dotenv::dotenv;
use log::info;

use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn load_env() {
    match dotenv().ok() {
        Some(_) => info!("Loaded .env file"),
        None => info!("No .env file found, assuming environment variables are set"),
    };
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_conn_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
