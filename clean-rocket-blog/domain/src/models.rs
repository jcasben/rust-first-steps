use crate::schema::posts;
use diesel::{Insertable, Queryable};
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub genre: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub genre: String,
}