use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::alphabet;
use base64::{engine, Engine};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fmt::Display;
use std::string::String;

#[derive(Debug, Clone)]
pub enum SecretError {
    Base64DecodeError(String),
    EncryptionError(String),
    DecryptionError(String),
}

impl Display for SecretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretError::Base64DecodeError(e) => write!(f, "Base64 decode error: {}", e),
            SecretError::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            SecretError::DecryptionError(e) => write!(f, "Decryption error: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeySecretPair {
    pub key: String,
    pub secret: String,
}

pub fn key_secret_pair() -> KeySecretPair {
    let key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    KeySecretPair { key, secret }
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::PAD);
    let result = engine.decode(input.as_bytes());
    result
}

pub fn encode_base64(input: &[u8]) -> String {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::PAD);
    engine.encode(input)
}

pub fn encrypt_string(input: &str, encryption_key: &str) -> Result<String, SecretError> {
    let key =
        decode_base64(encryption_key).map_err(|e| SecretError::Base64DecodeError(e.to_string()))?;
    let encrypted = encrypt_aes_256_gcm(input.as_bytes(), &key)
        .map_err(|e| SecretError::EncryptionError(e.to_string()))?;
    Ok(encode_base64(&encrypted))
}

pub fn decrypt_string(encrypted: &str, encryption_key: &str) -> Result<String, SecretError> {
    let key =
        decode_base64(encryption_key).map_err(|e| SecretError::Base64DecodeError(e.to_string()))?;
    let encrypted_bytes =
        decode_base64(encrypted).map_err(|e| SecretError::Base64DecodeError(e.to_string()))?;
    let decrypted_bytes = decrypt_aes_256_gcm(&encrypted_bytes, &key)
        .map_err(|e| SecretError::DecryptionError(e.to_string()))?;
    let decrypted_string = String::from_utf8(decrypted_bytes)
        .map_err(|e| SecretError::DecryptionError(e.to_string()))?;

    Ok(decrypted_string)
}

pub fn encrypt_aes_256_gcm(input: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    cipher.encrypt(&nonce, input).and_then(|ciphertext| {
        let mut result = Vec::new();
        result.extend_from_slice(nonce.as_ref());
        result.extend_from_slice(ciphertext.as_ref());
        Ok(result)
    })
}

pub fn decrypt_aes_256_gcm(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    if encrypted.len() < 12 {
        return Err(aes_gcm::Error);
    }
    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    cipher.decrypt(&nonce, ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::deployment_config::DeploymentConfig;

    #[test]
    fn test_key_secret_pairs() {
        let pair = key_secret_pair();
        assert_eq!(pair.key.len(), 10);
        assert_eq!(pair.secret.len(), 32);
    }

    #[test]
    fn test_decode_base64() {
        let input = "SGVsbG8gV29ybGQ=";
        let result = decode_base64(input);
        assert!(result.is_ok());
        let string_result = result.unwrap();
        let string_result = String::from_utf8(string_result);
        assert!(string_result.is_ok());
        assert_eq!(string_result.unwrap(), "Hello World");
    }

    #[test]
    fn test_encrypt_keypair() {
        let key_pair = key_secret_pair();
        let encryption_key = DeploymentConfig::load().encryption_key;
        let encryption_result = encrypt_string(key_pair.secret.as_str(), encryption_key.as_str());
        assert!(encryption_result.is_ok());
        let nonce_and_ciphertext = encryption_result.unwrap();
        let decryption_result =
            decrypt_string(nonce_and_ciphertext.as_str(), encryption_key.as_str());
        assert!(decryption_result.is_ok());
        assert_eq!(decryption_result.unwrap(), key_pair.secret);
    }

    #[test]
    fn test_encrypt_string() {
        let encryption_key = DeploymentConfig::load().encryption_key;
        let encrypted = encrypt_string("Hello World", encryption_key.as_str());
        assert!(encrypted.is_ok());
        let decrypted = decrypt_string(encrypted.unwrap().as_str(), encryption_key.as_str());
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), "Hello World");
    }
}
