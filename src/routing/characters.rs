use std::io;

use actix_web::{
    web::{self, Json, JsonBody, Path, Query},
    Error, HttpResponse,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::db::character_db::{Character, CharacterDB};

#[derive(Deserialize)]
pub struct GetCharactersQuery {
    limit: u32,
    start: u32,
}

pub async fn character(db: web::Data<Surreal<Client>>, character_id: Path<String>) -> HttpResponse {
    let result = CharacterDB::get_by_name(db.as_ref(), character_id.to_string()).await;

    match result {
        Ok(character) => HttpResponse::Ok().json(character),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

pub async fn characters(
    db: web::Data<Surreal<Client>>,
    query: Query<GetCharactersQuery>,
) -> HttpResponse {
    let result = CharacterDB::get_all(db.as_ref(), query.limit, query.start).await;

    match result {
        Ok(characters) => HttpResponse::Ok().json(characters),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

pub async fn create_character(
    db: web::Data<Surreal<Client>>,
    character: Json<Character>,
) -> HttpResponse {
    let result: Result<Character, surrealdb::Error> =
        CharacterDB::create(db.as_ref(), character.to_owned()).await;

    match result {
        Ok(created_character) => HttpResponse::Ok().json(created_character),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

pub async fn update_character(
    db: web::Data<Surreal<Client>>,
    character: Json<Character>,
) -> HttpResponse {
    let result: Result<Character, surrealdb::Error> =
        CharacterDB::update(db.as_ref(), character.to_owned()).await;

    match result {
        Ok(updated_character) => HttpResponse::Ok().json(updated_character),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

pub async fn remove_character(
    db: web::Data<Surreal<Client>>,
    character_id: Path<String>,
) -> HttpResponse {
    let result: Result<Character, surrealdb::Error> =
        CharacterDB::delete_by_name(db.as_ref(), character_id.to_owned()).await;

    match result {
        Ok(removed_character) => HttpResponse::Ok().json(removed_character),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
