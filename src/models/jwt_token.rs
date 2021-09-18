use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub exp: i64,
    pub iss: String,
    pub sub: String,
    pub username: String,
    pub email: String,
}

static ONE_WEEK_SECONDS: i64 = 60 * 60 * 24 * 7;
/// Add a reference to an array of bytes from a file with a secret key and size 16
pub static SECRET_KEY: [u8; 16] = *include_bytes!("../secret.key");

impl JwtToken {
    pub fn generate_token(username: String, email: String) -> String {
        // Convert ms to seconds
        let now = Utc::now().timestamp_millis() / 1000;

        // Customize algorithm
        let mut header = Header::default();
        header.alg = Algorithm::HS512;

        // Convert secret key to EncodingKey
        let encoding_key = EncodingKey::from_secret(&SECRET_KEY);

        // Setup payload
        let payload = JwtToken {
            exp: now + ONE_WEEK_SECONDS,
            iss: "rust-backend".to_string(),
            sub: "Authentication".to_string(),
            username,
            email,
        };

        // Encode JWT
        jsonwebtoken::encode(&header, &payload, &encoding_key).expect("JWT encoding error")
    }
}

#[cfg(test)]
mod test {
    use super::JwtToken;

    #[test]
    fn token_generation_test() {
        JwtToken::generate_token(
            "test_username".to_string(),
            "test_email@gmail.com".to_string(),
        );
    }
}
