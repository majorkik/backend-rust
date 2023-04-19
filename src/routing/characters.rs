use actix_web::{Error, HttpResponse};

pub async fn character() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn characters() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn create_character() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_character() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn remove_character() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
