extern crate rocket;

use rocket::{get, launch, post, routes};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Event {
    pub destination: String,
    pub body: String,
}

#[get("/")]
fn index() -> Json<String> {
    Json("Welcome to my API!".to_string())
}

#[post("/event", data = "<event>")]
fn receive_event(event: Json<Event>) -> String{
    println!("{:?}", event.body);

    format!("thanks {}", event.body)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![receive_event])
}