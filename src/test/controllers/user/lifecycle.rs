#[actix_web::test]
async fn test_user_lifecycle() {
    use crate::controllers::user::lifecycle::user_lifecycle;
    use crate::middleware::guard::auth_guard;
    use crate::structs::jwt::JwtManager;
    use crate::test::controllers::user::sign_in::perform_sign_in;
    use crate::test::helpers::init_jwt_envs;
    use actix_web::http::header;
    use actix_web::middleware::from_fn;
    use actix_web::{test, App};

    let tokens_payload = perform_sign_in().await;

    if tokens_payload.is_some() {
        init_jwt_envs();

        let tokens_payload = tokens_payload.unwrap();

        let app =
            test::init_service(App::new().wrap(from_fn(auth_guard)).service(user_lifecycle)).await;

        let req = test::TestRequest::get()
            .uri("/api/user/lifecycle")
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", tokens_payload.access_token),
            ))
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
    };
}
