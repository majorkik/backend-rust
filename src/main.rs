use actix_web::{get, HttpResponse, Responder};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware::Logger, App, HttpServer};

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().service(root).wrap(Logger::default()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Root endpoint")
}
