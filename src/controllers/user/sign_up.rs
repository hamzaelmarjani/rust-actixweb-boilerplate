use crate::constants::responder::global_responder;
use crate::constants::urls::API_BASE_URL;
use crate::controllers::user::tokanize_user::user_token_responder;
use crate::structs::jwt::JwtManager;
use crate::structs::request::user::{UserSignUPDTO, VerifyEmailPayload};
use crate::structs::response::PlaceholderResponse;
use crate::structs::user::User;
use crate::utils::validators::is_valid_email;
use actix_web::web::Json;
use actix_web::{post, Responder};
use uniqer_rs::Uniqer;

#[post("/api/user/sign-up")]
/// User's Sign-up Entry Point `/api/user/sign-up`.
/// The argument `body` is the parsed body as `UserSignUPDTO` struct.
/// No Middleware data argument here, because this route is public.
async fn user_sign_up(body: Json<UserSignUPDTO>) -> impl Responder {
    // 1. Replace the `PlaceholderResponse` struct.

    let mut response = PlaceholderResponse {
        data1: Some(serde_json::json!(false)),
        data2: None,
        data3: None,
        data4: None,
    };

    // 2. Find the user by his username or email.
    // If the user is not found, create the user. otherwise return if the email exists or the username exists.
    // I assume that the user does not exist, and I'll create a new one:

    // 3. Create a new unique user id using my crate `uniqer_rs` https://crates.io/crates/uniqer_rs.
    let uniquer_id = match Uniqer::new().unique_number() {
        Ok(id) => id,
        Err(_) => {
            // Enable to create a unique number id using `uniqer_rs` crate, so we'll panic.
            response.data1 = Some(serde_json::json!("internal-error"));
            return global_responder(500, &response);
        }
    };

    // 4. Create a new RFC 3339 and ISO 8601 date and time string using `chrono` crate.
    let created_at = chrono::Utc::now().to_rfc3339();

    // 5. Create a new user struct only if the email is a valid email address.
    if is_valid_email(&body.email) {
        let new_user = User {
            id: uniquer_id.clone(),
            username: body.username.clone(),
            name: body.name.clone(),
            email: body.email.clone(),
            created_at,
        };

        // 6. We will create a new link to the user and send it via email.
        let verify_email_payload = VerifyEmailPayload {
            id: uniquer_id,
            email: body.email.clone(),
        };

        // 7. The Verify Email Token will remain 2 hours for security.
        let user_email_username =
            format!("{}|{}", verify_email_payload.email, verify_email_payload.id);

        let verify_link_token = JwtManager::new()
            .unwrap()
            .encode_jwt(&user_email_username, Some(2 * 60 * 60))
            .unwrap_or(String::from(""));

        if !verify_link_token.is_empty() {
            // 8. We need to create the verify email link and send it to the user via email.
            let _link = format!("{API_BASE_URL}/api/verify/email?token={verify_link_token}");
            // TODO send the _link to the user via email.
        }

        // 9. Tokanize the user and return the tokens, open `user_token_responder` for more.
        // #![feature(box_into_inner)]
        user_token_responder(&new_user, &mut response, None)
    } else {
        response.data1 = Some(serde_json::json!("invalid-email-address"));
        global_responder(400, &response)
    }
}
