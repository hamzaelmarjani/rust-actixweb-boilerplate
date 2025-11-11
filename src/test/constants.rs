#[cfg(test)]
mod tests {
    use crate::constants::responder::global_responder;
    use crate::constants::routes::PUBLIC_ROUTES;
    use actix_web::body::MessageBody;

    #[test]
    fn test_global_response() {
        let response_body = serde_json::json!({"key": "value"});
        let response = global_responder(200, &response_body);
        assert_eq!(response.status(), 200);
        assert_ne!(response.status(), 401);

        let body1 = response.into_body();
        let bytes = body1.try_into_bytes().unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(body_json, response_body);
    }

    #[test]
    fn test_is_route_public() {
        let is_public = PUBLIC_ROUTES
            .iter()
            .any(|route_regex| route_regex.is_match("/api/user/lifecycle"));

        assert_eq!(is_public, false);
        assert_ne!(is_public, true);
    }
}
