use actix_web::{App, HttpServer};
use routing::config_service;

mod routing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config_service))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
