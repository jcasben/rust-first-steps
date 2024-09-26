use crate::ServerState;
use diesel::{QueryDsl, RunQueryDsl};
use domain::models::Post;
use rocket::response::status::NotFound;
use rocket::State;
use shared::response_models::{Response, ResponseBody};

pub fn list_post(state: &State<ServerState>, post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts;

    match posts::table.find(post_id).first::<Post>(&mut state.db_pool.get().unwrap()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!("Error searching post with id {} - {}", post_id, err))
                };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => { panic!("Database error - {}", err); }
        }
    }
}

pub fn list_posts(state: &State<ServerState>) -> Vec<Post> {
    use domain::schema::posts;

    match posts::table.select(posts::all_columns).load::<Post>(&mut state.db_pool.get().unwrap()) {
        Ok(mut posts) => {
            posts.sort();
            posts
        }
        Err(err) => panic!("Database error - {}", err)
    }
}