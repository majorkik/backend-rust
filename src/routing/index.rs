use actix_web::{HttpResponse, Responder};

pub async fn index_route() -> impl Responder {
    HttpResponse::Ok().body("The server is working correctly!")
}
