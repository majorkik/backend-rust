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
    use self::super::JwtToken;
    use crate::utils::jwt_token::decode_jwt_token;
    use jsonwebtoken::errors::ErrorKind;
    use jsonwebtoken::{Algorithm, Validation};

    #[test]
    fn test_token_decoding() {
        let jwt_token =
            JwtToken::generate_token("test_username".to_string(), "test@gmail.com".to_string());

        let result_decoding = match decode_jwt_token(jwt_token) {
            Ok(val) => val,
            Err(err) => panic!(err.to_string()),
        };
    }
}
