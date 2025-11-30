use std::{fs, path::Path};

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Key,
    Nonce,
};
use anyhow::{anyhow, Result};
use argon2::Argon2;
use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroize;

const MAGIC: &[u8; 4] = b"ZV1!";
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

/// Derive a 32-byte key from password + salt using Argon2id.
fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let argon2 = Argon2::default();

    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("Argon2 key derivation failed: {e}"))?;

    Ok(key)
}

/// Encrypt `input_path` → `output_path` using ZeroVault format.
pub fn encrypt_file_with_password(
    input_path: &Path,
    output_path: &Path,
    password: &str,
) -> Result<()> {
    // Read plaintext
    let plaintext = fs::read(input_path)?;

    // Generate random salt + nonce
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    let mut rng = OsRng;

    rng.fill_bytes(&mut salt);
    rng.fill_bytes(&mut nonce_bytes);

    // Derive key
    let mut key_bytes = derive_key_from_password(password, &salt)?;

    // Build cipher
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .map_err(|e| anyhow!("Encryption failed: {e}"))?;

    // Zero key material
    key_bytes.zeroize();

    // Build output: MAGIC || SALT || NONCE || CIPHERTEXT
    let mut out = Vec::with_capacity(
        MAGIC.len() + SALT_LEN + NONCE_LEN + ciphertext.len(),
    );
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);

    fs::write(output_path, out)?;

    Ok(())
}

/// Decrypt a ZeroVault file at `input_path` to plaintext at `output_path`.
pub fn decrypt_file_with_password(
    input_path: &Path,
    output_path: &Path,
    password: &str,
) -> Result<()> {
    let data = fs::read(input_path)?;

    if data.len() < MAGIC.len() + SALT_LEN + NONCE_LEN {
        return Err(anyhow!("Input too short to be a ZeroVault file"));
    }

    // Parse header
    let (magic, rest) = data.split_at(MAGIC.len());
    if magic != MAGIC {
        return Err(anyhow!("Not a ZeroVault file (magic mismatch)"));
    }

    let (salt, rest) = rest.split_at(SALT_LEN);
    let (nonce_bytes, ciphertext) = rest.split_at(NONCE_LEN);

    // Derive key
    let mut key_bytes = derive_key_from_password(password, salt)?;

    // Build cipher
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt
    let mut plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Decryption failed (wrong password or corrupted file)"))?;

    // Zero key bytes
    key_bytes.zeroize();

    // Write plaintext out
    fs::write(output_path, &plaintext)?;

    // Wipe plaintext buffer
    plaintext.zeroize();

    Ok(())
}