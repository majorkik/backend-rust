use chrono;

#[get("/")]
pub fn get_home() -> String {
    let current_utc_date_time = chrono::offset::Utc::now();
    let formatted_date_time = current_utc_date_time.format("%e %a %Y %T");

    let content = "The server is running, so it's a lucky day!\n\nCurrent date & time";

    return format!("{}: {}", content, formatted_date_time);
}
