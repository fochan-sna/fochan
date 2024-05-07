#[macro_use] extern crate rocket;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use rocket::serde::{Deserialize, json::Json};
use rocket::State;

struct MyState {
    names: Arc<Mutex<Vec<String>>>
}

#[get("/")]
fn index(state: &State<MyState>) -> String {
    let mut result_string = String::new();
    let names_mutex = state.names.lock().unwrap();
    let names = names_mutex.deref();
    for name in names {
        result_string.push_str(name.as_str());
    }
    result_string
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: i32
}

#[post("/", format="json", data = "<user>")]
fn post_index(user: Json<User>, state: &State<MyState>) {
    let mut names = state.names.lock().unwrap();
    names.push(user.name.clone());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post_index])
        .manage(MyState { names: Arc::new(Mutex::new(Vec::new()))})
}