use actix_web::{web, App, HttpServer};
use routing::config_service;
use std::env::{self, VarError};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

mod db;
mod routing;

struct DBConfig {
    username: String,
    password: String,
}

fn load_db_config() -> Result<DBConfig, VarError> {
    let username = env::var("surrealdb_username")?;
    let password = env::var("surrealdb_pass")?;

    Ok(DBConfig { username, password })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Checking that .the env file exists
    dotenvy::dotenv().expect(".env file not found");

    // Loading configs
    let config = load_db_config().unwrap();

    let client = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Error connecting to SurrealDB!");

    // Setup SurrealDB
    client
        .signin(Root {
            username: &config.username,
            password: &config.password,
        })
        .await
        .expect("Invalid credentials for SurrealDB");

    client.use_ns("test").use_db("test").await.unwrap();

    client
        .health()
        .await
        .expect("An error occurred while configuring SurrealDB");

    // Setup Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(config_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
