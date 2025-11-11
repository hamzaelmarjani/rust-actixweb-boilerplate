use crate::structs::jwt::TokensPayload;

pub async fn perform_sign_in() -> Option<TokensPayload> {
    std::env::set_var("JWT_SECRET", "u5LdfHsuS1xYxZ8FSg9X5fY3o5w8Rv5MG7ZkUPHTkB4=");
    std::env::set_var(
        "ENCRYPTION_SECRET",
        "Q/ifjccW09g6ZgkQ8HUN1YUVSgGbeDRO6R4bXqjM1V8=",
    );

    use crate::controllers::user::sign_in::user_sign_in;
    use actix_web::{test, App};
    use serde_json::json;
    let app = test::init_service(App::new().service(user_sign_in)).await;

    let arg = json!({
        "username": "mr_elmarjani",
        "password": "Hamza00Elmarjani@@",
    });

    let req = test::TestRequest::post()
        .uri("/api/user/sign-in")
        .set_json(&arg)
        .to_request();

    let resp = test::call_service(&app, req).await;

    if resp.status() == 200 {
        let body: serde_json::Value = test::read_body_json(resp).await;

        let tokens = TokensPayload {
            access_token: Some(body["data3"]["access_token"].clone())
                .unwrap_or(json!(""))
                .to_string(),
            refresh_token: Some(body["data3"]["refresh_token"].clone())
                .unwrap_or(json!(""))
                .to_string(),
        };

        println!("Tokens: {:?}", tokens);

        return Some(tokens);
    };

    None
}

#[actix_web::test]
async fn test_user_sign_in() {
    use crate::structs::jwt::JwtManager;

    let tokens_payload = perform_sign_in().await;

    if tokens_payload.is_some() {
        let tokens_payload = tokens_payload.unwrap();

        assert_eq!(
            JwtManager::new()
                .unwrap()
                .verify(&tokens_payload.access_token)
                .unwrap()
                .expired,
            false
        );

        assert_eq!(
            JwtManager::new()
                .unwrap()
                .verify(&tokens_payload.refresh_token)
                .unwrap()
                .expired,
            false
        );
    } else {
        panic!("Tokens are not valid!");
    }
}
