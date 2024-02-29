use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    DbPool::builder().build(manager).expect("Failed to create pool")
}
