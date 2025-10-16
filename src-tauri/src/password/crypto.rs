use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, Salt, SaltString},
    Argon2, Params,
};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng as ChaChaRng},
    ChaCha20Poly1305, Nonce,
};
use std::error::Error;

pub fn derive_key(password: &str, salt: &[u8; 32]) -> Result<[u8; 32], Box<dyn Error>> {
    let params = Params::new(65536, 3, 4, Some(32))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let salt_str = SaltString::encode_b64(salt).map_err(|e| e.to_string())?;
    let salt = Salt::from_b64(&salt_str).map_err(|e| e.to_string())?;

    let hash = argon2
        .hash_password(password.as_bytes(), salt)
        .map_err(|e| e.to_string())?;

    let hash_bytes = hash.hash.ok_or("Failed to get hash bytes")?;
    let mut key = [0u8; 32];
    key.copy_from_slice(hash_bytes.as_bytes());

    Ok(key)
}

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let nonce = ChaCha20Poly1305::generate_nonce(&mut ChaChaRng);

    let ciphertext = cipher
        .encrypt(&nonce, data)
        .map_err(|e| format!("Encryption error: {}", e))?;

    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    if encrypted_data.len() < 12 {
        return Err("Encrypted data too short".into());
    }

    let cipher = ChaCha20Poly1305::new(key.into());

    let nonce = Nonce::from_slice(&encrypted_data[..12]);

    let ciphertext = &encrypted_data[12..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption error: {}", e))?;

    Ok(plaintext)
}

pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    use rand::RngCore;
    OsRng.fill_bytes(&mut salt);
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let password = "test_password_12345";
        let salt = generate_salt();
        let key = derive_key(password, &salt).unwrap();

        let plaintext = b"Hello, World! This is a secret message.";
        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_wrong_password() {
        let password1 = "correct_password";
        let password2 = "wrong_password";
        let salt = generate_salt();

        let key1 = derive_key(password1, &salt).unwrap();
        let key2 = derive_key(password2, &salt).unwrap();

        let plaintext = b"Secret data";
        let encrypted = encrypt(plaintext, &key1).unwrap();

        let result = decrypt(&encrypted, &key2);
        assert!(result.is_err());
    }

    #[test]
    fn test_deterministic_key_derivation() {
        let password = "same_password";
        let salt = generate_salt();

        let key1 = derive_key(password, &salt).unwrap();
        let key2 = derive_key(password, &salt).unwrap();

        assert_eq!(key1, key2);
    }
}
