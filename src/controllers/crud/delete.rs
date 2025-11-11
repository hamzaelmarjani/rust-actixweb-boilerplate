use crate::constants::responder::global_responder;
use crate::structs::middleware::MiddlewareExtensionsData;
use crate::structs::request::crud::CrudDeleteDataDTO;
use crate::structs::response::PlaceholderResponse;
use actix_web::web::{Json, ReqData};
use actix_web::{delete, Responder};

#[delete("/api/crud/delete")]
/// Crud's Delete-Data Entry Point `/api/crud/delete`.
/// The argument `_middleware_data` is coming from the `middleware` filter inside: `src/middleware/guard.rs`.
/// The argument `body` is the parsed body as `CrudDeleteDataDTO` struct.
async fn delete_data(
    _middleware_data: ReqData<MiddlewareExtensionsData>,
    body: Json<CrudDeleteDataDTO>,
) -> impl Responder {
    // 1. Get your data from a database

    let data1 = serde_json::json!("Data from Database");
    let mut data_from_db: Vec<serde_json::Value> = Vec::new();
    for _ in 0..100 {
        data_from_db.push(data1.clone());
    }

    // 2. Make your delete operation

    data_from_db.retain(|x| x != &body.arg);

    // 3. Replace the `PlaceholderResponse` struct

    let response = PlaceholderResponse {
        data1: Some(data_from_db.into()),
        data2: None,
        data3: None,
        data4: None,
    };

    // 4. Return the response as `Responder` included the `response` struct and 200 success status code

    global_responder(200, &response)
}
