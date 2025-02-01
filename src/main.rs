mod disk_writer;

extern crate rocket;

use std::sync::Mutex;
use rocket::{get, launch, post, routes, State};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use crate::disk_writer::DiskWriter;

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
// Route to receive events and write them to disk
#[post("/event", data = "<event>")]
fn receive_event(event: Json<Event>, disk_writer: &State<Mutex<DiskWriter>>) -> &'static str {
    let writer = disk_writer.inner().lock().unwrap(); // Acquire a mutable guard
    if let Err(e) = writer.write_item(&event.into_inner()) {
        eprintln!("Failed to write event to disk: {}", e);
        return "Error writing event to disk";
    }
    if let Err(e) = writer.flush() {
        eprintln!("Failed to flush data to disk: {}", e);
        return "Error flushing data to disk";
    }
    "Event received and written to disk!"
}


#[launch]
fn rocket() -> _ {
    // Initialize the DiskWriter
    let disk_writer = DiskWriter::new("output.txt").expect("Failed to create DiskWriter");

    // Wrap DiskWriter in a Mutex and pass it as managed state
    rocket::build()

        .manage(Mutex::new(disk_writer))
            .mount("/", routes![index])
        .mount("/", routes![receive_event])
}

