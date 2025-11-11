use actix_web::body::BoxBody;
use actix_web::{dev::ServiceResponse, Error as ActixWebError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NextResultResponder = Result<ServiceResponse<BoxBody>, ActixWebError>;

/// A Global Response across all the app, use it to don't repeat your-self.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResponse<T> {
    pub message: Option<String>,
    pub data: Option<T>,
}

/// Just a Placeholder Response `struct` for demonstration only, you can replace it with your own `struct`.
/// Make sure your own `struct` has the following macros: `Serialize`, `Deserialize` and `Clone`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceholderResponse {
    pub data1: Option<serde_json::Value>,
    pub data2: Option<Vec<serde_json::Value>>,
    pub data3: Option<HashMap<String, serde_json::Value>>,
    pub data4: Option<Vec<HashMap<String, serde_json::Value>>>,
}
