#[macro_use]
extern crate rocket;

use chrono;

use rocket::{Build, Rocket};

#[get("/")]
fn home() -> String {
    let current_utc_date_time = chrono::offset::Utc::now();
    let formatted_date_time = current_utc_date_time.format("%e %a %Y %T");

    let content = "The server is running, so it's a lucky day!\n\nCurrent date & time";

    return format!("{}: {}", content, formatted_date_time);
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![home])
}
