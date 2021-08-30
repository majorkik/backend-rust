use crate::models::user::User;
use crate::schema::users;

use diesel::{prelude::*, result::Error};
use serde::Deserialize;

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub avatar_url: &'a str,
    pub quot: &'a str,
}

fn create_user(
    conn: &PgConnection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, Error> {
    let new_user = NewUser {
        username,
        email,
        password_hash: password,
        avatar_url: "",
        quot: "",
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)?;

    Ok(result)
}
