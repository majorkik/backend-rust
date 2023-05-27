use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Result, Surreal};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub status: String,
}

pub struct CharacterDB;

impl CharacterDB {
    pub async fn get_all(db: &Surreal<Client>, limit: u32, start: u32) -> Result<Vec<Character>> {
        let mut response = db
            .query("SELECT * FROM character LIMIT $limit START $start")
            .bind(("limit", limit))
            .bind(("start", start))
            .await?;
        let characters: Vec<Character> = response.take(0)?;

        dbg!(&characters);

        Ok(characters)
    }

    pub async fn get_by_name(db: &Surreal<Client>, name: String) -> Result<Character> {
        let character: Character = db.select(("character", name)).await?;

        dbg!(&character);

        Ok(character)
    }

    pub async fn create(db: &Surreal<Client>, character: Character) -> Result<Character> {
        let created: Character = db
            .create(("character", &character.name))
            .content(character)
            .await?;

        dbg!(&created);

        Ok(created)
    }

    pub async fn update(db: &Surreal<Client>, character: Character) -> Result<Character> {
        let response: Character = db
            .update(("character", &character.name))
            .merge(character)
            .await?;

        dbg!(&response);

        Ok(response)
    }

    pub async fn delete_by_name(db: &Surreal<Client>, name: String) -> Result<Character> {
        let deleted: Character = db.delete(("character", name)).await?;

        Ok(deleted)
    }
}
