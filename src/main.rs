use rocket::{get, launch, routes};
use rocket::serde::json::Json;

#[get("/")]
fn index() -> Json<String> {
    Json("Welcome to my API!".to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}