use crate::structs::jwt::JwtManager;
use crate::utils::validators::is_valid_email;
use actix_web::{get, web::Html, HttpRequest, Responder};
use askama::Template;

/// I'll use `askama` crate to render the HTML success template using the user `name` and `email`.
/// All HTML templates are stored inside the `templates` folder in the project root.
/// Success HTML template is under: `templates/pages/user/email_verification/success.html`.
/// `SuccessHtmlTemplate` includes the `name` and `email` values to render the final HTML text dynamically.
#[derive(Template)]
#[template(path = "pages/user/email_verification/success.html")]
pub struct SuccessHtmlTemplate<'a> {
    pub email: &'a str,
    pub name: &'a str,
}

/// I'll use `askama` crate to render the success HTML template.
/// All HTML templates are stored inside the `templates` folder in the project root.
/// Failed HTML template is under: `templates/pages/user/email_verification/failed.html`.
/// This HTML template doesn't require any variables.
#[derive(Template)]
#[template(path = "pages/user/email_verification/failed.html")]
pub struct FailedHtmlTemplate {}

#[get("/api/verify/email")]
/// User's Email Verification Entry Point `/api/verify/email`.
async fn user_email_verification(req: HttpRequest) -> impl Responder {
    // 1. Extract the `token` from the query string.
    let token: String = req
        .match_info()
        .query("token")
        .parse()
        .unwrap_or(String::from(""));

    if token.is_empty() {
        return failed_to_verify_email();
    };

    // 2. Decode & verify the token, then extract the email from the token.
    let decoded_token = JwtManager::new().unwrap().decode_jwt_and_decrypt(&token);
    let payload_str = decoded_token.unwrap_or_else(|_| String::from(""));

    if payload_str.is_empty() {
        return failed_to_verify_email();
    }

    let (email, username) = payload_str.split_once("|").unzip();
    if email.is_none() || username.is_none() || !is_valid_email(&email.unwrap()) {
        // Return failed ...
    }

    // 3. Check if the email and username are valid,
    // Here you need to check those on your database or your Authentication provider.
    // I'll assume that the email and username are valid for now:

    // Also, get the user's real Name from the database so we can inject it to the HTML text.

    let name = String::from("Hamza El Marjani");
    success_to_verify_email(&email.unwrap(), &name)
}

fn success_to_verify_email(email: &str, name: &str) -> Html {
    Html::new(
        SuccessHtmlTemplate { name, email }
            .render()
            .unwrap_or(String::from("Success")),
    )
}

fn failed_to_verify_email() -> Html {
    Html::new(
        FailedHtmlTemplate {}
            .render()
            .unwrap_or(String::from("Failed")),
    )
}
