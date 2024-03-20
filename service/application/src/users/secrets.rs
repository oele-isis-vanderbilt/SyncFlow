use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use hex::encode;
use rand;
use rand::RngCore;

pub fn generate_key_pair_from_hex_key(
    encryption_key_hex: &str,
) -> Result<(String, String), aes_gcm::Error> {
    decode_hex(encryption_key_hex)
        .map_err(|e| aes_gcm::Error)
        .and_then(|key| generate_key_pair(&key))
}

pub fn decrypt_secret(secret_key: &str, encryption_key: &str) -> Result<String, aes_gcm::Error> {
    decode_hex(secret_key)
        .map_err(|e| aes_gcm::Error)
        .and_then(|key| {
            decode_hex(encryption_key)
                .map_err(|e| aes_gcm::Error)
                .and_then(|encryption_key| decrypt(&key, &encryption_key))
                .map(|s| String::from_utf8(s).unwrap())
        })
}

/// Generate a new key pair for
pub fn generate_key_pair(encryption_key: &[u8]) -> Result<(String, String), aes_gcm::Error> {
    let mut key = [0u8; 8];
    let mut secret = [0u8; 16];

    rand::thread_rng().fill_bytes(&mut key);
    rand::thread_rng().fill_bytes(&mut secret);
    let encryption_result = encrypt(&secret, encryption_key)?;

    Ok((encode(&key), encode(&encryption_result)))
}

pub fn decode_hex(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

fn encrypt(secret: &[u8], encryption_key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let key = Key::<Aes256Gcm>::from_slice(encryption_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(&key);
    let ciphertext = cipher.encrypt(&nonce, secret)?;
    let nonce_and_ciphertext = nonce
        .to_vec()
        .into_iter()
        .chain(ciphertext.into_iter())
        .collect();
    Ok(nonce_and_ciphertext)
}

fn decrypt(nonce_and_ciphertext: &[u8], encryption_key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    if nonce_and_ciphertext.len() < 12 {
        // Check for nonce size
        return Err(aes_gcm::Error);
    }
    let key = Key::<Aes256Gcm>::from_slice(encryption_key);
    let (nonce, ciphertext) = nonce_and_ciphertext.split_at(12);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    cipher.decrypt(nonce, ciphertext)
}

mod tests {
    use super::{decrypt, encrypt, generate_key_pair};
    use hex::decode;

    #[test]
    fn test_encrypt_decrypt() {
        let encryption_key_hex =
            std::env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
        let encryption_key = decode(encryption_key_hex).unwrap();
        let secret = "MySecret".as_bytes();
        let encrypted_and_nonce = encrypt(secret, &encryption_key).unwrap();
        let decrypted = decrypt(&encrypted_and_nonce, &encryption_key).unwrap();
        let decrypted = String::from_utf8(decrypted).unwrap();
        assert_eq!("MySecret", decrypted);
    }
}
