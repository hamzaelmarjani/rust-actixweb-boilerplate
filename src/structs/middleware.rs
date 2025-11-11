use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MiddlewareExtensionsData {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
}
