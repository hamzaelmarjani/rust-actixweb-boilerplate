use serde::{Deserialize, Serialize};

/// A `User` struct to be serialized, you can add your own user struct here.
/// Make sure the struct has the following macros `Serialize`, `Deserialize` and `Clone`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

impl User {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
