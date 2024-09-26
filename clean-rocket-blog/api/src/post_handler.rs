use application::post::{create, delete, publish, read};
use domain::models::{NewPost, Post};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post, State};
use application::ServerState;
use shared::response_models::{Response, ResponseBody};

#[get("/")]
pub fn list_posts_handler(state: &State<ServerState>) -> String {
    let posts: Vec<Post> = read::list_posts(state);
    let response = Response { body: ResponseBody::Posts(posts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/post/<post_id>")]
pub fn list_post_handler(state: &State<ServerState>, post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::list_post(state, post_id)?;
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_post", format = "application/json", data = "<post>")]
pub fn create_post_handler(state: &State<ServerState>, post: Json<NewPost>) -> Created<String> {
    create::create_post(state, post)
}

#[get("/publish/<post_id>")]
pub fn publish_post_handler(state: &State<ServerState>, post_id: i32) -> Result<String, NotFound<String>> {
    let post = publish::publish_post(state, post_id)?;
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/delete/<post_id>")]
pub fn delete_post_handler(state: &State<ServerState>, post_id: i32) -> Result<String, NotFound<String>> {
    let posts = delete::delete_post(state, post_id)?;
    let response = Response { body: ResponseBody::Posts(posts) };

    Ok(serde_json::to_string(&response).unwrap())
}