#[macro_use] extern crate rocket;

use diesel::sql_types::Uuid;
use rocket::serde::{Serialize, Deserialize, json::Json};

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
fn get_user_id() -> Json<GetUserIdResponse> {

}

#[get("/topics")]
fn get_topics() -> Json<GetTopicsResponse> {

}

#[get("/get_messages")]
fn get_messages() -> Json<GetMessagesResponse> {

}

#[post("/write_message", format = "json", data = "<request>")]
fn post_message(request: Json<PostMessageRequest>) {

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