/*
* JWT Manager, a module that handles JWT operations:
*  —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— ——
* | 1. Encoding                                      |
* | 2. Decoding                                      |
* | 3. Verification                                  |
* | 4. Encryption                                    |
* | 5. Decryption                                    |
*  —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— ——
* This module provides a comprehensive set of functions for working with JWT tokens.
*/

use crate::{
    structs::jwt::{Claims, JwtManager, VerifyResult},
    utils::env::get_env,
};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{anyhow, Error, Result};
use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

impl JwtManager {
    /// Creates a new instance of JwtManager
    pub fn new() -> Result<Self> {
        // Get the secret key from the environment variable `JWT_SECRET`
        let secret_key = get_env("JWT_SECRET");

        // Get the encryption password from the environment variable `ENCRYPTION_SECRET`
        let encryption_secret = get_env("ENCRYPTION_SECRET");

        Ok(Self {
            secret_key,
            encryption_secret,
        })
    }

    /// Derives a 32-byte key from the encryption password
    fn derive_key(&self) -> [u8; 32] {
        let mut hasher = Sha256::default();
        hasher.update(self.encryption_secret.as_bytes());
        hasher.finalize().into()
    }

    /// Encrypt the JWT payload using AES-256-GCM encryption algorithm
    #[allow(deprecated)]
    pub fn encrypt_data(&self, data: &str) -> Result<String> {
        let binding = self.derive_key();
        let key = Key::<Aes256Gcm>::from_slice(&binding);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(result))
    }

    /// Decrypt the JWT payload using AES-256-GCM decryption algorithm
    #[allow(deprecated)]
    pub fn decrypt_data(&self, encrypted_data: &str) -> Result<String> {
        let data = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|e| anyhow!("Base64 decode failed: {}", e))?;

        if data.len() < 12 {
            return Err(anyhow!("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let binding = self.derive_key();
        let key = Key::<Aes256Gcm>::from_slice(&binding);
        let cipher = Aes256Gcm::new(key);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| anyhow!("UTF-8 conversion failed: {}", e))
    }

    /// Encode and Sign the JWT string using HS256 algorithm
    pub fn encode_jwt(&self, payload: &str, exp_secs: Option<u64>) -> Result<String> {
        let encrypted_payload = self.encrypt_data(payload)?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // Expiration time default to 24 Hours if `exp_secs` is not provided
        let exp = now + exp_secs.unwrap_or(24 * 60 * 60);

        let claims = Claims {
            sub: encrypted_payload,
            exp: exp as usize,
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.secret_key.as_ref());

        encode(&header, &claims, &encoding_key).map_err(|e| anyhow!("JWT encoding failed: {}", e))
    }

    /// Decode the JWT string and return the claims
    pub fn decode_jwt(&self, token: &str) -> Result<TokenData<Claims>> {
        let decoding_key = DecodingKey::from_secret(self.secret_key.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false; // Don't validate expiration

        decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| anyhow!("JWT decoding failed: {}", e))
    }

    /// Shortcut function to decode and decrypt the JWT in same time and return the claims.
    pub fn decode_jwt_and_decrypt(&self, token: &str) -> Result<String, Error> {
        let verify_token = self.verify(token)?;
        if verify_token.expired {
            return Err(Error::msg("token-expired"));
        }

        let decoded = self.decode_jwt(&token)?;
        let decrypted = self
            .decrypt_data(&decoded.claims.sub)
            .unwrap_or("".to_string());
        Ok(decrypted)
    }

    /// Sign the JWT string using HS256 algorithm
    pub fn sign_jwt(&self, token: &str) -> Result<TokenData<Claims>> {
        let decoding_key = DecodingKey::from_secret(self.secret_key.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| anyhow!("JWT verification failed: {}", e))
    }

    /// Verify the JWT string signature using the HS256 algorithm and check expiration
    pub fn verify(&self, token: &str) -> Result<VerifyResult> {
        let token_data = match self.sign_jwt(token) {
            Ok(_) => {
                return Ok(VerifyResult { expired: false });
            }
            Err(_) => self.decode_jwt(token)?,
        };

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;

        let is_expired = token_data.claims.exp < current_time;

        Ok(VerifyResult {
            expired: is_expired,
        })
    }
}
