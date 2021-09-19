use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

impl JwtToken {
    pub fn has_role(&self, role: String) -> bool {
        self.roles.contains(&role.to_string())
    }
}

#[cfg(test)]
mod test {

    use crate::utils::jwt_token::generate_token;

    #[test]
    fn token_generation_test() {
        generate_token(
            "test_username".to_string(),
            "test_email@gmail.com".to_string(),
            vec![String::from("user")],
        );
    }
}
