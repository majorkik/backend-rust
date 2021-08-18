#[macro_use]
extern crate rocket;
use rocket::{Build, Rocket};

mod routes;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![routes::home::get_home])
}
