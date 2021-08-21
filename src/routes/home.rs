use chrono;

#[get("/")]
pub fn get_home() -> String {
    let current_utc_date_time = chrono::offset::Utc::now();
    let formatted_date_time = current_utc_date_time.format("%e %a %Y %T");

    let content = "The server is running, so it's a lucky day!\n\nCurrent date & time";

    return format!("{}: {}", content, formatted_date_time);
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_get_home() {
        let client = Client::tracked(rocket()).expect("valid rocket");
        let response = client.get("/").dispatch();

        // Check that the request succeeded
        assert_eq!(response.status(), Status::Ok);
        // Check that the query is returning a string
        assert_eq!(response.body().is_none(), false);
    }
}
