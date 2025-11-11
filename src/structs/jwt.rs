use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct VerifyResult {
    pub expired: bool,
}

pub struct JwtManager {
    pub secret_key: String,
    pub encryption_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokensPayload {
    pub access_token: String,
    pub refresh_token: String,
}
