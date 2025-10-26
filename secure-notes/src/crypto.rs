use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};

const NONCE_SIZE: usize = 12;

/// Derives a 32-byte encryption key from a password using Argon2
pub fn derive_key(password: &str, salt: &str) -> Result<[u8; 32], String> {
    let argon2 = Argon2::default();
    let salt = SaltString::from_b64(salt).map_err(|e| e.to_string())?;
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;
    
    let hash = password_hash.hash.ok_or("Failed to get hash")?;
    let hash_bytes = hash.as_bytes();
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash_bytes[..32]);
    Ok(key)
}

/// Generates a new random salt for key derivation
pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

/// Encrypts data using ChaCha20Poly1305
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let cipher = ChaCha20Poly1305::new(key.into());
    
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Decrypts data using ChaCha20Poly1305
pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    if encrypted_data.len() < NONCE_SIZE {
        return Err("Invalid encrypted data".to_string());
    }
    
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let cipher = ChaCha20Poly1305::new(key.into());
    
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}

/// Encodes binary data to base64 string
pub fn encode_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Decodes base64 string to binary data
pub fn decode_base64(data: &str) -> Result<Vec<u8>, String> {
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| e.to_string())
}
