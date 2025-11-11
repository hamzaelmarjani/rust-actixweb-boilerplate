use crate::constants::responder::global_responder;
use crate::structs::middleware::MiddlewareExtensionsData;
use crate::structs::request::crud::CrudCreateDataDTO;
use crate::structs::response::PlaceholderResponse;
use actix_web::web::{Json, ReqData};
use actix_web::{post, Responder};
use std::collections::HashMap;

#[post("/api/crud/create")]
/// Crud's Create-Data Entry Point `/api/crud/create`.
/// The argument `_middleware_data` is coming from the `middleware` filter inside: `src/middleware/guard.rs`.
/// The argument `body` is the parsed body as `CrudCreateDataDTO` struct.
async fn create_data(
    _middleware_data: ReqData<MiddlewareExtensionsData>,
    body: Json<CrudCreateDataDTO>,
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

    // 3. Make your crate operation

    if let Some(_) = data_from_db.iter_mut().find(|m| {
        m.values()
            .any(|v| v.as_str() == Some(&body.arg.iter().next().unwrap().1).unwrap().as_str())
    }) {
        // 4. If the item already exists, return the response as `Responder` included the `response` struct and 400 bad-request status code
        response.data1 = Some(serde_json::json!("item-already-exists"));
        global_responder(400, &response)
    } else {
        data_from_db.push(body.arg.clone());
        // 5.If the item not exists, add it and return the response as `Responder` included the `response` struct and 200 success status code
        response.data1 = Some(serde_json::json!(true));
        global_responder(200, &response)
    }
}
