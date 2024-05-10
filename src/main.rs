#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

// use diesel::associations::HasTable;
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
    let database_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST is not set");
    let database_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT is not set");
    let database_name = env::var("POSTGRES_DB").expect("POSTGRES_DB is not set");
    let database_user = env::var("POSTGRES_USER").expect("POSTGRES_USER is not set");
    let database_pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD is not set");
    let database_url = format!("postgres://{}:{}@{}:{}/{}", database_user, database_pass, database_host, database_port, database_name);
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

#[derive(Serialize, Deserialize, Queryable)]
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


#[derive(Serialize, Deserialize)]
struct GetMessagesRequest {
    topic_id: Uuid,
    limit: i64
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

#[get("/get_messages", format = "json", data = "<request>")]
fn get_messages(request: Json<GetMessagesRequest>) -> Result<Json<GetMessagesResponse>> {
    use self::schema::messages::dsl::*;

    let connection = &mut establish_connection_pg();
    let topic_ID = request.topic_id.clone();
    let limit = request.limit;

    let results = messages
        .select((user_id, content))
        .filter(topic_id.eq(topic_ID))
        .order(sent_at.desc())
        .limit(limit)
        .load::<Message>(connection)
        .expect("Error while fetching messages");

    let response = GetMessagesResponse {
        messages: results
    };
    
    Ok(Json(response))
}


#[post("/write_message", format = "json", data = "<request>")]
fn post_message(request: Json<PostMessageRequest>) -> Result<Created<String>> {
    Ok(Created::new("The message was written"))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let mut config = rocket::Config::default();
    config.address = env::var("ROCKET_HOST")
        .unwrap_or("127.0.0.1".parse().unwrap()).parse().unwrap();
    config.port = env::var("ROCKET_PORT")
        .unwrap_or("8000".parse().unwrap()).parse().unwrap();

    rocket::build()
        .configure(config)
        .mount("/", routes![
            get_user_id,
            get_topics,
            get_messages,
            post_message
        ])
}