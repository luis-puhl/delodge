use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    name: String,
    description: String,
}

pub type LocationList = Vec<Location>;

#[get("/locations", format = "json")]
pub fn locations() -> Json<LocationList> {
    let locations: LocationList = vec![
        Location {
            name: "São Paulo - Pinheiros".into(),
            description: "Nice bars".into(),
        },
        Location {
            name: "São Paulo - Liberdade".into(),
            description: "Nice restaurants".into(),
        },
    ];
    return Json(locations);
}
