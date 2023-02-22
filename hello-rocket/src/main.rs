use std::io;
use std::option::Option;
use std::path::{PathBuf, Path};

#[macro_use] extern crate rocket;
use rocket::fs::{NamedFile, relative};
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;

#[get("/<path>")]
async fn index(path: String) -> Option<NamedFile> {
    let mut path: PathBuf = Path::new(relative!("static")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
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

// #[derive(Deserialize)]
// struct Task { name: String, completed: bool }

// #[post("/", data = "<task>")]
// fn new(task: Json<Task>) -> Flash<Redirect> {
//     if task.name.is_empty() {
//         Flash::error(Redirect::to("/"),
//             "Cannot be empty.")
//     } else {
//         Flash::success(Redirect::to("/"),
//             "Task added.")
//     }
// }


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello_world, delay, blocking_task])
}
