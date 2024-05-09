use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use super::schema::posts;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}