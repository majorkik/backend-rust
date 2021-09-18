use crate::models::jwt_token::{JwtToken, SECRET_KEY};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};

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

    use super::JwtToken;
    use crate::utils::jwt_token::decode_jwt_token;

    #[test]
    fn test_token_decoding() {
        let jwt_token =
            JwtToken::generate_token("test_username".to_string(), "test@gmail.com".to_string());

        match decode_jwt_token(jwt_token) {
            Ok(val) => val,
            Err(err) => panic_any(err.to_string()),
        };
    }
}
