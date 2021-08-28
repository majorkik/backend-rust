use diesel::types::Int4;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatarUrl: String,
    pub quot: String,
}