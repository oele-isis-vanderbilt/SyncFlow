use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.")
}
