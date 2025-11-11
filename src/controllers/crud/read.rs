use std::collections::HashMap;

use crate::constants::responder::global_responder;
use crate::structs::middleware::MiddlewareExtensionsData;
use crate::structs::response::PlaceholderResponse;
use actix_web::web::ReqData;
use actix_web::{get, Responder};

#[get("/api/crud/read")]
/// Crud's Read-Data Entry Point `/api/crud/read`.
/// The argument `_middleware_data` is coming from the `middleware` filter inside: `src/middleware/guard.rs`
async fn read_data(_middleware_data: ReqData<MiddlewareExtensionsData>) -> impl Responder {
    // 1. Get your data from a database

    let data1 = serde_json::json!("Data from Database");

    let data2 = [data1.clone(), data1.clone()].to_vec();

    let mut data3 = HashMap::new();
    data3.insert("key1".to_string(), serde_json::json!("Data from Database"));
    data3.insert("key2".to_string(), serde_json::json!(1500));
    data3.insert("key3".to_string(), serde_json::json!(true));

    let data4 = [data3.clone(), data3.clone()].to_vec();

    // 2. Replace the `PlaceholderResponse` struct

    let response = PlaceholderResponse {
        data1: Some(data1),
        data2: Some(data2),
        data3: Some(data3),
        data4: Some(data4),
    };

    // 3. Return the response as `Responder` included the `response` struct and 200 success status code

    global_responder(200, &response)
}
