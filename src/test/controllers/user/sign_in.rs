use crate::structs::jwt::TokensPayload;

#[allow(dead_code)]
pub async fn perform_sign_in() -> Option<TokensPayload> {
    use crate::test::helpers::init_jwt_envs;
    init_jwt_envs();

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
            access_token: Some(body["data3"]["access_token"].clone().as_str())
                .unwrap_or(Some(""))
                .unwrap()
                .to_string(),
            refresh_token: Some(body["data3"]["refresh_token"].clone().as_str())
                .unwrap_or(Some(""))
                .unwrap()
                .to_string(),
        };

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
