use crate::controllers::user::combinator::combine_user_by_id;
use crate::controllers::user::tokanize_user::user_token_responder;
use crate::structs::middleware::MiddlewareExtensionsData;
use crate::structs::response::PlaceholderResponse;
use actix_web::web::ReqData;
use actix_web::{get, Responder};

#[get("/api/user/lifecycle")]
/// User's Lifecycle Entry Point `/api/user/lifecycle` to check the user access_token and refresh_token.
/// The argument `middleware_data` is coming from the `middleware` filter inside: `src/middleware/guard.rs`.
async fn user_lifecycle(middleware_data: ReqData<MiddlewareExtensionsData>) -> impl Responder {
    // 1. Replace the `PlaceholderResponse` struct.

    let mut response = PlaceholderResponse {
        data1: Some(serde_json::json!(false)),
        data2: None,
        data3: None,
        data4: None,
    };

    // 2. While you are here, The Middleware Auth Guard filter is already checked the request access_token and refresh_token.
    // So you just need to return a new access_token and the same refresh_token.

    response.data1 = Some(serde_json::json!(true));

    // 4. Tokanize the user and return the tokens, open `user_token_responder` for more.
    user_token_responder(
        &combine_user_by_id(middleware_data.user_id.clone()),
        &mut response,
        middleware_data.refresh_token.clone(),
    )
}
