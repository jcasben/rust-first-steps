#[macro_use] extern crate rocket;
use api::post_handler;
use application::{get_pool, PgPool, ServerState};

#[launch]
fn rocket() -> _ {
    let connection_pool: PgPool = get_pool();
    rocket::build()
        .mount("/api", routes![
            post_handler::list_posts_handler,
            post_handler::list_post_handler,
            post_handler::create_post_handler,
            post_handler::publish_post_handler,
            post_handler::delete_post_handler
        ])
        .manage(ServerState { db_pool: connection_pool })
}