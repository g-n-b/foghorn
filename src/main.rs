mod disk_writer;

extern crate rocket;

use crate::disk_writer::DiskWriter;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes, State};
use std::sync::Mutex;

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

// Route to receive events and write them to disk
/// # Arguments
///
/// * `event` - the Json payload of an @Event type
/// * `disk_writer` - The disk writer singleton
///
#[post("/event", data = "<event>")]
fn receive_event(event: Json<Event>, disk_writer: &State<Mutex<DiskWriter>>) -> Result<String, String> {
    let writer = disk_writer.inner().lock().unwrap(); // Acquire a mutable guard
    if let Err(e) = writer.write_item(&event.into_inner()) {
        eprintln!("Failed to write event to disk: {}", e);
        // TODO how does Err work?
        return Err(String::from("Error"));
    }
    if let Err(e) = writer.flush() {
        eprintln!("Failed to flush data to disk: {}", e);
        return Err("Internal Error".to_string());
    }
    Ok("Event received and written to disk!".to_string())
}

#[launch]
fn rocket() -> _ {
    // Initialize the DiskWriter
    let disk_writer = DiskWriter::new("output.txt").expect("Failed to create DiskWriter");

    // Wrap DiskWriter in a Mutex and pass it as managed state
    rocket::build()
        // Store the diskwriter as a singleton to be managed by rocket
        .manage(Mutex::new(disk_writer))
        .mount("/", routes![index])
        .mount("/", routes![receive_event])
}
