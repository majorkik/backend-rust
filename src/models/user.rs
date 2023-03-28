use diesel;
use serde::{Serialize, Deserialize};
use diesel::{prelude::*, QueryDsl, result::Error};
use crate::{
    schema::users::{self, dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub quot: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn login(conn: &mut PgConnection, login: LoginDTO) -> Option<User> {
        let user: Result<User, Error> = users
            .filter(username.eq(&login.username))
            .get_result::<User>(conn);

        match user {
            Ok(val) => {
                if !val.password_hash.is_empty()
                    && verify(&login.password_hash, &val.password_hash).unwrap() {

                    // TODO: Add JWT token generation logic

                    Some(val)
                } else {
                    Some(val)
                }
            }

            Result::Err(_err) => None,
        }
    }
}