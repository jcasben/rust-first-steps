use std::env;
use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env must be set!");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn pool_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env must be set!");
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Couldn't create a connection pool")
}