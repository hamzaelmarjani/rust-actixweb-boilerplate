use crate::constants::responder::global_responder;
use crate::structs::jwt::JwtManager;
use crate::structs::response::PlaceholderResponse;
use crate::structs::user::User;
use actix_web::HttpResponse;
use std::collections::HashMap;

pub fn user_token_responder(
    user: &User,
    response: &mut PlaceholderResponse,
    refreshed_token: Option<String>,
) -> HttpResponse {
    // 1. We need to create an access_token.
    let access_token = JwtManager::new()
        .unwrap()
        .encode_jwt(&user.to_string(), None)
        .unwrap_or(String::from(""));

    // 2. If the refresh_token doesn't exist, we need to create it again with a long expiration time (30 Days)
    let refresh_token = if refreshed_token.is_none() {
        JwtManager::new()
            .unwrap()
            .encode_jwt(&user.to_string(), Some(60 * 60 * 24 * 30))
            .unwrap_or(String::from(""))
    } else {
        refreshed_token.unwrap()
    };

    if access_token.is_empty() || refresh_token.is_empty() {
        // 3. If the access_token or refresh_token is empty, return the response as `Responder` included the `response` struct and 500 internal-error status code
        response.data1 = Some(serde_json::json!("intrernal-error"));
        global_responder(500, response)
    } else {
        // 4. Return the response as `Responder` included the `response` struct and 200 success status code
        let mut tokens_map: HashMap<String, serde_json::Value> = HashMap::new();
        tokens_map.insert("access_token".to_string(), serde_json::json!(access_token));
        tokens_map.insert("refresh_token".to_string(), serde_json::json!(refresh_token));
        response.data1 = Some(serde_json::json!(&true));
        response.data3 = Some(tokens_map);
        global_responder(200, response)
    }
}
