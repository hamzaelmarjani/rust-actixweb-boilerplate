use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MiddlewareExtensionsData {
    pub user_id: String,
    pub refresh_token: Option<String>,
}
