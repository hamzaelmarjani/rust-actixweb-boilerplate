pub mod user;

// #[actix_web::test]
// async fn test_crud_create_data() {
//     std::env::set_var("JWT_SECRET", "u5LdfHsuS1xYxZ8FSg9X5fY3o5w8Rv5MG7ZkUPHTkB4=");
//     std::env::set_var(
//         "ENCRYPTION_SECRET",
//         "Q/ifjccW09g6ZgkQ8HUN1YUVSgGbeDRO6R4bXqjM1V8=",
//     );
//
//     use crate::controllers::crud::create::create_data;
//     use crate::structs::middleware::MiddlewareExtensionsData;
//     use actix_web::{test, App};
//     use serde_json::json;
//
//     let app = test::init_service(App::new().service(create_data)).await;
//
//     let payload = json!({
//         "foo": "bar",
//     });
//
//     // Note: Create a request with mock middleware data, the access_token and refresh_token should be valid and retrieved from the sign-in test method.
//     let middleware_data = MiddlewareExtensionsData {
//         user_id: "mr_elmarjani".to_string(),
//         // In most cases, those tokens are expired, try to get a new one from the sign-in test method.
//         access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJpWWxVdFJKd3BnamFEWit0MHlYZ3hYMTVSREtYUExRcXhhL0tTalkyYzhsSytPUmpVeWFZWU9zWTBONVNpYUhsdUlxTGJSeHpjeWhLcUNCTXJNd0JNNmE5QTZhalV4QjM4SFh1QUgwSU5hYmZNcWJVdTd1elhUVFZDUTNLQkR2RzhoN01BMXE0UWE5VXBLY2NkYVo2eXdHNmNlR0o1RUhZY3hPcGE2ckFVZ1FmU2NCV2FBYnVPN2MxbFdad2dQU2RpSlJPWVVudld5RTBhQjNjRE1vN3pMWTkzaHNEUE1nM2RLRUtGMkJyalNnPSIsImV4cCI6MTc2Mjk1MTczNn0.OK5i2k0qUlkyPhCIO_UsxziGB0ySKKc-Vkql7OCyDKY".to_string(),
//         refresh_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJnd2ZtTTl5S2ZCT2pwcWw1NmhNWEc0YnRaaXZUOFFYZU03TkFuSGFoYTMrb2E2ZHZ3WFByR29KSk9vZXdPa29aQkRwQzhaanlOcHJ4cEFaNzI0dEp1bWswYkVyRVNYWEhvbERUQzgzcVQwOENjbWRLdmN2TGtZY2U5MGZuTjUzNUhycEJiV0hGNjFIYmdyNnBBMld5Vzd2Y2o5ZnBPMnNYUkpnQUVKM1NpQkZSRll1ZWdRTUdHdHIveDNnb25mTnN0WWZUd3h4YmNjaTJOSFh5YlhMcWhBYmdJSGN0aGVTWGZnWThaVGN1MjNrPSIsImV4cCI6MTc2NTQ1NzMzNn0.DGPBT8JnJXdRHQn8dDfj3pqesPFNo-GxeVDnXoySaqI".to_string(),
//     };
//
//     let req = test::TestRequest::post()
//         .uri("/api/crud/create")
//         .set_json(&payload)
//         .data(middleware_data)
//         .to_request();
//
//     let resp = test::call_service(&app, req).await;
//
//     assert_eq!(resp.status(), 200);
//
//     let body: serde_json::Value = test::read_body_json(resp).await;
//     assert_eq!(body["data1"], true);
//     assert_ne!(body["data2"], "NOT_EXISTED_DATA");
// }
