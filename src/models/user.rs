use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub quot: Option<String>,
}
