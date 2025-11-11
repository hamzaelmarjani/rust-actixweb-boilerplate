use crate::constants::responder::global_responder;
use crate::structs::middleware::MiddlewareExtensionsData;
use crate::structs::request::crud::CrudUpdateDataDTO;
use crate::structs::response::PlaceholderResponse;
use actix_web::web::{Json, ReqData};
use actix_web::{patch, Responder};
use std::collections::HashMap;

#[patch("/api/crud/update")]
/// Crud's Update-Data Entry Point `/api/crud/update`.
/// The argument `_middleware_data` is coming from the `middleware` filter inside: `src/middleware/guard.rs`.
/// The argument `body` is the parsed body as `CrudUpdateDataDTO` struct.
async fn update_data(
    _middleware_data: ReqData<MiddlewareExtensionsData>,
    body: Json<CrudUpdateDataDTO>,
) -> impl Responder {
    // 1. Replace the `PlaceholderResponse` struct
    let mut response = PlaceholderResponse {
        data1: Some(serde_json::json!(false)),
        data2: None,
        data3: None,
        data4: None,
    };

    // 2. Get your data from a database

    let data1 = serde_json::json!("Data from Database");
    let mut data_from_db: Vec<HashMap<String, serde_json::Value>> = Vec::new();
    for i in 0..100 {
        let mut hashmap = HashMap::new();
        hashmap.insert(i.to_string(), data1.clone());
        data_from_db.push(hashmap);
    }

    // 3. Make your update operation

    if let Some(found) = data_from_db
        .iter_mut()
        .find(|map| map.values().any(|v| v.as_str() == Some(&body.arg1)))
    {
        *found = body.arg2.clone();
        response.data1 = Some(serde_json::json!(true));
    }

    // 4. Return the response as `Responder` included the `response` struct and 200 success status code

    global_responder(200, &response)
}
