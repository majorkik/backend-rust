use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Result, Error};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Character {
    id: u32,
    name: String,
}

struct Repository {
    characters: HashMap<u32, Character>,
}

impl Repository {
    fn new() -> Self {
        Self {
            characters: HashMap::new(),
        }
    }
}

#[get("/")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("The server is working correctly!")
}

async fn create_character(character: web::Json<Character>, data: web::Data<Repository>) -> Result<String> { 
    Ok(format!("New character: {}!", character.name))
}

#[get("/character/{charcter_id}")]
async fn get_character(character_id: web::Path<u32>, data: web::Data<Repository>) -> Result<impl Responder> {
    let character: Option<&Character> = data.characters.get(&character_id);

    match character {
        Some(character) => Ok(web::Json(character.clone())),
        None => panic!(), // TODO: Add error handling
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Repository::new()))
            .service(echo)
            .service(get_character)
            .route("/character", web::post().to(create_character))
        }
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
