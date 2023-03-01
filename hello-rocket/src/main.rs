use std::io::{self, Result};

#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};

use crate::api::locations;

mod api;

#[get("/world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> Result<String> {
    sleep(Duration::from_secs(seconds)).await;
    return Ok(format!("Waited for {} seconds", seconds))
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

mod static_files {
    use rocket::response::content;
    use rocket::Request;

    #[catch(404)]
    pub fn not_found(req: &Request<'_>) -> content::RawHtml<String> {
        content::RawHtml(format!(
            r#"<p>Sorry, but we could not find '{}'</p>
            <a href="/">Take me home</a>"#,
            req.uri()
        ))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .register("/", catchers![static_files::not_found])
        .mount(
            "/api",
            routes![hello_world, delay, blocking_task, locations],
        )
}
