use actix_web::{App, HttpServer};
use routing::config_service;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

mod routing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Error connecting to SurrealDB!");

    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .expect("Invalid credentials for SurrealDB");

    client
        .health()
        .await
        .expect("An error occurred while configuring SurrealDB");

    HttpServer::new(|| App::new().configure(config_service))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
