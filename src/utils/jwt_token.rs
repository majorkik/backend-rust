use crate::models::jwt_token::JwtToken;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};

static JWT_EXPIRATION_TIME: i64 = 60 * 60 * 24 * 7;
/// Add a reference to an array of bytes from a file with a secret key and size 16
pub static SECRET_KEY: [u8; 16] = *include_bytes!("../secret.key");

pub fn generate_token(username: String, email: String, roles: Vec<String>) -> String {
    // Convert ms to seconds
    let now = Utc::now().timestamp_millis() / 1000;

    // Customize algorithm
    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    // Convert secret key to EncodingKey
    let encoding_key = EncodingKey::from_secret(&SECRET_KEY);

    // Setup payload
    let payload = JwtToken {
        iat: now,
        exp: now + JWT_EXPIRATION_TIME,
        iss: "rust-backend".to_string(),
        username,
        email,
        roles,
    };

    // Encode JWT
    jsonwebtoken::encode(&header, &payload, &encoding_key).expect("JWT encoding error")
}

pub fn decode_jwt_token(token: String) -> jsonwebtoken::errors::Result<TokenData<JwtToken>> {
    jsonwebtoken::decode::<JwtToken>(
        &token,
        &DecodingKey::from_secret(&SECRET_KEY),
        &Validation::new(Algorithm::HS512),
    )
}

#[cfg(test)]
mod test {
    use std::panic::panic_any;

    use super::*;

    #[test]
    fn test_token_decoding() {
        let jwt_token = generate_token(
            "test_username".to_string(),
            "test@gmail.com".to_string(),
            vec![String::from("user")],
        );

        match decode_jwt_token(jwt_token) {
            Ok(val) => val,
            Err(err) => panic_any(err.to_string()),
        };
    }
}
