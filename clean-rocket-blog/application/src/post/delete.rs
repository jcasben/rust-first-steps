use crate::ServerState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use domain::models::Post;
use rocket::response::status::NotFound;
use rocket::State;
use shared::response_models::{Response, ResponseBody};

pub fn delete_post(state: &State<ServerState>, post_id: i32) -> Result<Vec<Post>, NotFound<String>> {
    use domain::schema::posts::dsl::*;
    use domain::schema::posts;

    let response: Response;

    let num_deleted = match diesel::delete(posts::table.filter(id.eq(post_id))).execute(&mut state.db_pool.get().unwrap()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting post with id {} - {}", post_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    };

    if num_deleted > 0 {
        match posts::table.select(posts::all_columns).load::<Post>(&mut state.db_pool.get().unwrap()) {
            Ok(mut posts_) => {
                posts_.sort();
                Ok(posts_)
            }
            Err(err) => {
                panic!("Database error - {}", err);
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no post with id {}", post_id)) };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}