use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod post;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct ServerState {
    pub db_pool: PgPool
}

pub fn get_pool() -> PgPool {
    infrastructure::pool_connection()
}