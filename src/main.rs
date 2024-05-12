#![feature(lazy_cell)]
#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

pub mod models;
pub mod schema;

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
use std::collections::HashMap;
use ws::{WebSocket, Stream};
use crossbeam::channel;
use std::sync::LazyLock;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
type TopicChannelMap = LazyLock<HashMap<Uuid, (channel::Sender<models::Message>, channel::Receiver<models::Message>)>>;

static GLOBAL_CHANNELS: TopicChannelMap = LazyLock::new(|| {
    let mut channels = HashMap::new();

    use self::schema::topics::dsl::*;
    let connection = &mut establish_connection_pg();

    let query_topics = topics
    .select(topic_id)
    .load::<Uuid>(connection)
    .expect("Error while fetching topics");

    for topic in query_topics {
        let (sender, receiver) = channel::unbounded();
        channels.insert(topic, (sender, receiver));
    }

    channels
});

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct GetTopicsResponse {
    topics: Vec<Topic>
}

#[derive(Serialize, Deserialize, Queryable)]
struct Message {
    message_id: i32,
    username: String,
    content: String,
    sent_at: NaiveDateTime,
    topic_id: Uuid
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

    let results = topics
        .select(Topic::as_select())
        .load::<Topic>(connection)
        .expect("Error while fetching topics");

    let response = GetTopicsResponse {
        topics: results
    };
    Ok(Json(response))
}

#[get("/get_messages?<limit>")]
fn get_messages(limit: i64) -> Result<Json<GetMessagesResponse>> {
    use self::schema::messages::dsl::*;
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    let results = messages
        .inner_join(users::table().on(schema::messages::user_id.eq(schema::users::user_id)))
        .select((schema::messages::message_id, username, content, sent_at, topic_id))
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

#[get("/get_messages_stream?<topic_id>")]
fn get_messages_stream(topic_id: String, _ws: WebSocket) -> Stream!['static] {
    Stream! { _ws => {
        let topic_id = Uuid::parse_str(topic_id.as_str()).unwrap();
        let channel = GLOBAL_CHANNELS.get(&topic_id).unwrap();
        let receiver = channel.1.clone();

        loop {
            let message = receiver.recv().unwrap();
            let json_message = serde_json::to_string(&message).unwrap();
            yield ws::Message::Text(json_message);
        }
    }}
}

#[post("/write_message", format = "json", data = "<request>")]
fn post_message(request: Json<PostMessageRequest>) -> Result<Created<String>> {
    use self::schema::messages::dsl::*;

    let connection = &mut establish_connection_pg();

    let insert_time = Local::now().naive_local();

    diesel::insert_into(messages)
       .values((
            topic_id.eq(request.topic_id),
            user_id.eq(request.user_id),
            content.eq(request.message.clone()),
            sent_at.eq(insert_time),
        ))
        .execute(connection)
        .expect("Error while inserting message into DB");

    let message = messages
        .select(models::Message::as_select())
        .filter(sent_at.eq(insert_time))
        .load(connection)
        .expect("Error while reading message from DB");

    let channel = GLOBAL_CHANNELS.get(&request.topic_id).unwrap();
    let sender = channel.0.clone();
    sender.send(message[0].clone()).expect("Message wasn't sent to socket");

    Ok(Created::new("The message was written"))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let mut config = rocket::Config::default();
    config.address = env::var("ROCKET_HOST")
        .unwrap_or("0.0.0.0".parse().unwrap()).parse().unwrap();
    config.port = env::var("ROCKET_PORT")
        .unwrap_or("8000".parse().unwrap()).parse().unwrap();

    rocket::build()
        .configure(config)
        .mount("/", routes![
            get_user_id,
            get_topics,
            get_messages,
            post_message,
            get_messages_stream
        ])
}