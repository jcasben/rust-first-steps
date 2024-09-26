use crate::ServerState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use domain::models::Post;
use rocket::response::status::NotFound;
use rocket::State;
use shared::response_models::{Response, ResponseBody};

pub fn publish_post(state: &State<ServerState>, post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts::dsl::*;
    
    match diesel::update(posts.find(post_id)).set(published.eq(true)).get_result::<Post>(&mut state.db_pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing post with id {} - {}", post_id, err)) };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}