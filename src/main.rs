#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::serde::json::Json;
use dotenvy::dotenv;
use std::env;
use rocket::response::Debug;
use rocket::response::status::Created;
use uuid::Uuid;
use rand::random;
use serde::{Serialize, Deserialize};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_host = env::var("DATABASE_HOST").expect("DATABASE_HOST is not set");
    let database_port = env::var("DATABASE_PORT").expect("DATABASE_PORT is not set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set");
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER is not set");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set");
    let database_url = format!("postgres://{}:{}@{}:{}/{}", database_user, database_password, database_host, database_port, database_name);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
struct GetUserIdResponse {
    user_id: Uuid,
    username: String
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
    let response = GetUserIdResponse {
        user_id: Uuid::new_v4(),
        username: random::<u32>().to_string()
    };
    Ok(Created::new("/get_user_id").body(Json(response)))
}

#[get("/get_topics")]
fn get_topics() -> Result<Json<GetTopicsResponse>> {
    use self::schema::topics::dsl::*;

    let connection = &mut establish_connection_pg();

    let results: Vec<String> = topics
        .select(name)
        .load(connection)
        .expect("Error while fetching topics");

    let response = GetTopicsResponse {
        topics: results
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