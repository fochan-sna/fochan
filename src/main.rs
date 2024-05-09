#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize, json::Json};
use dotenvy::dotenv;
use std::env;
use rocket::response::Debug;
use rocket::response::status::Created;
use uuid::Uuid;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
struct GetUserIdResponse {
   user_id: Uuid
}

#[derive(Serialize, Deserialize)]
struct GetTopicsResponse {
    topics: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Message {
    user_id: Uuid,
    context: String
}

#[derive(Serialize, Deserialize)]
struct GetMessagesResponse {
    messages: Vec<Message>
}

#[derive(Serialize, Deserialize)]
struct PostMessageRequest {
    user_id: Uuid,
    topic_id: String,
    message: String
}

#[get("/get_user_id")]
fn get_user_id() -> Result<Created<Json<GetUserIdResponse>>> {
    let response = GetUserIdResponse { user_id: Uuid::new_v4() };
    Ok(Created::new("/get_user_id").body(Json(response)))
}

#[get("/topics")]
fn get_topics() -> Result<Json<GetTopicsResponse>> {
    let response = GetTopicsResponse {
        topics: Vec::new()
    };
    Ok(Json(response))
}

#[get("/get_messages")]
fn get_messages() -> Result<Json<GetMessagesResponse>> {
    let response = GetMessagesResponse {
        messages: Vec::new()
    };
    Ok(Json(response))
}

#[post("/write_message", format = "json", data = "<request>")]
fn post_message(request: Json<PostMessageRequest>) -> Result<Created<String>> {
    Ok(Created::new("The message was written"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_user_id,
            get_topics,
            get_messages,
            post_message
        ])
}