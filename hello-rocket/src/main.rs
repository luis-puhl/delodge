use std::io;

#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Location { name: String, description: String }

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct LocationList { seq: Vec<Location> }

#[get("/locations", format = "json")]
fn locations() -> Json<LocationList> {
    let locations = LocationList{
        seq: vec![
        Location {
            name: "São Paulo - Pinheiros".into(),
            description: "Nice bars".into(),
        },
        Location {
            name: "São Paulo - Liberdade".into(),
            description: "Nice restaurants".into(),
        }
    ]};
    Json(locations);
}

#[get("/world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![hello_world, delay, blocking_task, locations])
}
