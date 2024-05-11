#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::serde::json::Json;
use dotenvy::dotenv;
use std::env;
use chrono::Local;
use rocket::response::Debug;
use rocket::response::status::Created;
use uuid::Uuid;
use rand::random;
use serde::{Serialize, Deserialize};
use models::*;
use chrono::NaiveDateTime;

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

type GetUserIdResponse = User;

#[derive(Serialize, Deserialize)]
struct GetTopicsResponse {
    topics: Vec<Topic>
}

#[derive(Serialize, Deserialize, Queryable)]
struct Message {
    username: String,
    content: String,
    sent_at: NaiveDateTime
}

#[derive(Serialize, Deserialize)]
struct GetMessagesResponse {
    messages: Vec<Message>
}

#[derive(Serialize, Deserialize)]
struct PostMessageRequest {
    user_id: Uuid,
    topic_id: Uuid,
    message: String
}



#[get("/get_user_id")]
fn get_user_id() -> Result<Created<Json<GetUserIdResponse>>> {
    use crate::schema::users;

    let connection = &mut establish_connection_pg();

    let response = GetUserIdResponse {
        user_id: Uuid::new_v4(),
        username: random::<u32>().to_string()
    };

    diesel::insert_into(users::table)
        .values(&response)
        .execute(connection)
        .expect("Error while inserting the user");

    Ok(Created::new("/get_user_id").body(Json(response)))
}

#[get("/get_topics")]
fn get_topics() -> Result<Json<GetTopicsResponse>> {
    use self::schema::topics::dsl::*;

    let connection = &mut establish_connection_pg();

    let results = topics .select(Topic::as_select()) .load::<Topic>(connection)
        .expect("Error while fetching topics");

    let response = GetTopicsResponse {
        topics: results
    };
    Ok(Json(response))
}

#[get("/get_messages?<topic>&<limit>")]
fn get_messages(topic: String, limit: i64) -> Result<Json<GetMessagesResponse>> {
    use self::schema::messages::dsl::*;
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    let results = messages
        .inner_join(users::table().on(schema::messages::user_id.eq(schema::users::user_id)))
        .select((username, content, sent_at))
        // .filter(topic_id.eq(Uuid::parse_str(topic.as_str()).unwrap()))
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
    use crate::schema::messages;

    let connection = &mut establish_connection_pg();
    
    let message = models::Message {
        message_id: Uuid::new_v4(),
        topic_id: request.topic_id,
        user_id: request.user_id,
        content: request.message.clone(),
        sent_at: Local::now().naive_local()
    };

    diesel::insert_into(messages::table)
        .values(&message)
        .execute(connection)
        .expect("Failed to insert user");

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