#[actix_web::test]
async fn test_user_sign_up() {
    use crate::controllers::user::sign_up::user_sign_up;
    use crate::structs::jwt::JwtManager;
    use actix_web::{test, App};
    use serde_json::json;
    let app = test::init_service(App::new().service(user_sign_up)).await;

    let arg = json!({
        "name": "Hamza El Marjani",
        "username": "mr_elmarjani",
        "email": "hamzaelmarjani@gmail.com",
        "password": "Hamza00Elmarjani@@",
    });

    let req = test::TestRequest::post()
        .uri("/api/user/sign-up")
        .set_json(&arg)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    if resp.status() == 200 {
        let body: serde_json::Value = test::read_body_json(resp).await;

        assert_eq!(
            JwtManager::new()
                .unwrap()
                .verify(
                    &Some(body["data3"]["access_token"].clone().as_str())
                        .unwrap_or(Some(""))
                        .unwrap()
                        .to_string()
                )
                .unwrap()
                .expired,
            false
        );

        assert_eq!(
            JwtManager::new()
                .unwrap()
                .verify(
                    &Some(body["data3"]["refresh_token"].clone().as_str())
                        .unwrap_or(Some(""))
                        .unwrap()
                        .to_string()
                )
                .unwrap()
                .expired,
            false
        );
    };
}
