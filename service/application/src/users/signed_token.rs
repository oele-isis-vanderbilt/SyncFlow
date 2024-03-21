use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SignedTokenError {
    JWTError(String),
}

impl Display for SignedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignedTokenError::JWTError(e) => write!(f, "JWT error: {}", e),
        }
    }
}

pub fn generate_and_sign_jwt<T>(claims: &T, secret: &str) -> Result<String, SignedTokenError>
where
    T: serde::Serialize,
{
    let secret = secret.to_string();
    encode(
        &Header {
            alg: Algorithm::HS256,
            ..Header::default()
        },
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| SignedTokenError::JWTError(e.to_string()))
}

pub fn verify_and_decode_jwt<T: DeserializeOwned>(
    token: &str,
    secret: &str,
) -> Result<T, SignedTokenError> {
    let secret = secret.to_string();
    let token_data = decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| SignedTokenError::JWTError(e.to_string()))?;
    Ok(token_data.claims)
}

pub fn decode_jwt_unsafe<T: DeserializeOwned>(token: &str) -> Result<T, SignedTokenError> {
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    let token_data = decode::<T>(token, &key, &validation)
        .map_err(|e| SignedTokenError::JWTError(e.to_string()))?;
    Ok(token_data.claims)
}

mod tests {
    use super::{decode_jwt_unsafe, generate_and_sign_jwt, verify_and_decode_jwt};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use serde::{Deserialize, Serialize};
    use std::char;

    fn random_string() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
    struct TestClaims {
        pub iat: usize,
        pub exp: usize,
        pub iss: String,
        pub company: String,
        pub api_key: String,
    }

    #[test]
    fn test_generate_and_sign_jwt() {
        let claims = TestClaims {
            iat: 10000000000,
            exp: 10000000000,
            iss: "test".to_string(),
            company: "test".to_string(),
            api_key: "test".to_string(),
        };
        let secret = random_string();
        let token = generate_and_sign_jwt::<TestClaims>(&claims, &secret);
        assert!(token.is_ok());
        let token_str = token.unwrap();
        let decoded = verify_and_decode_jwt::<TestClaims>(&token_str, &secret);
        assert!(decoded.is_ok());
        let decoded_claims = decoded.unwrap();
        assert_eq!(decoded_claims, claims);
        let decoded_unsafe = decode_jwt_unsafe::<TestClaims>(&token_str);
        assert!(decoded_unsafe.is_ok());
        let decoded_unsafe_claims = decoded_unsafe.unwrap();
        assert_eq!(decoded_unsafe_claims, claims);
    }
}
