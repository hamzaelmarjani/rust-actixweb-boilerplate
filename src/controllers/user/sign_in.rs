use crate::controllers::user::combinator::combine_user_by_id;
use crate::controllers::user::tokanize_user::user_token_responder;
use crate::structs::request::user::UserSignInDTO;
use crate::structs::response::PlaceholderResponse;
use actix_web::web::Json;
use actix_web::{post, Responder};

#[post("/api/user/sign-in")]
/// User's Sign-in Entry Point `/api/user/sign-in`.
/// The argument `_body` is the parsed body as `UserSignInDTO` struct.
/// No Middleware data argument here, because this route is public.
async fn user_sign_in(_body: Json<UserSignInDTO>) -> impl Responder {
    // 1. Replace the `PlaceholderResponse` struct.

    let mut response = PlaceholderResponse {
        data1: Some(serde_json::json!(false)),
        data2: None,
        data3: None,
        data4: None,
    };

    // 2. Sign in with your Authentication Provider using the `body.username` & `body.password`
    // The `body.username` can be a unique username or an email to get the user info from a database.
    // I assume that the user is authenticated, and you get the user info from the database for now:

    // 3. Note: you can email the user's email address to notify him that someone is trying to sign in.

    // 4. Tokanize the user and return the tokens, open `user_token_responder` for more.
    user_token_responder(
        &combine_user_by_id(String::from("790505571683")),
        &mut response,
        None,
    )
}
