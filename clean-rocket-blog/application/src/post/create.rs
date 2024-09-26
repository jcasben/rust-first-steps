use crate::ServerState;
use diesel::RunQueryDsl;
use domain::models::{NewPost, Post};
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::State;
use shared::response_models::{Response, ResponseBody};

pub fn create_post(state: &State<ServerState>, post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;

    let post = post.into_inner();

    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(&mut state.db_pool.get().unwrap()) {
        Ok(post) => {
            let response = Response { body: ResponseBody::Post(post) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}