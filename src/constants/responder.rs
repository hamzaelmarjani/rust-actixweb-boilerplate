use crate::structs::response::NextResultResponder;
use actix_web::{dev::ServiceRequest, HttpResponse, HttpResponseBuilder};
use serde::{de::DeserializeOwned, Serialize};

/// Just a Don't Repeat Your-Self global function that returns `Responder` with `status_code` and `response_body`.
pub fn global_responder<T>(status_code: i32, response_body: &T) -> HttpResponse
where
    T: DeserializeOwned + Serialize,
{
    let mut builder = http_response_builder(&status_code);

    builder
        .content_type("application/json")
        .json(serde_json::json!(&response_body))
}

/// Just a Don't Repeat Your-Self global function that returns `NextResultResponder` with `status_code` and `response_body`.
pub fn global_auth_guard_right_body<T>(
    req: ServiceRequest,
    status_code: i32,
    response_body: &T,
) -> NextResultResponder
where
    T: DeserializeOwned + Serialize,
{
    let mut builder = http_response_builder(&status_code);
    Ok(req.into_response(
        builder
            .json(serde_json::json!(response_body))
            .map_into_boxed_body(),
    ))
}

/// A function that returns a `HttpResponseBuilder` based on the `status_code`.
/// I handle all the standard common status codes available, you can add yours below.
fn http_response_builder(status_code: &i32) -> HttpResponseBuilder {
    let builder: HttpResponseBuilder;

    match status_code {
        // 2xx Success
        201 => builder = HttpResponse::Created(),
        202 => builder = HttpResponse::Accepted(),
        204 => builder = HttpResponse::NoContent(),

        // 3xx Redirection
        301 => builder = HttpResponse::MovedPermanently(),
        302 => builder = HttpResponse::Found(),
        304 => builder = HttpResponse::NotModified(),
        307 => builder = HttpResponse::TemporaryRedirect(),
        308 => builder = HttpResponse::PermanentRedirect(),

        // 4xx Client Errors
        400 => builder = HttpResponse::BadRequest(),
        401 => builder = HttpResponse::Unauthorized(),
        402 => builder = HttpResponse::PaymentRequired(),
        403 => builder = HttpResponse::Forbidden(),
        404 => builder = HttpResponse::NotFound(),
        405 => builder = HttpResponse::MethodNotAllowed(),
        406 => builder = HttpResponse::NotAcceptable(),
        408 => builder = HttpResponse::RequestTimeout(),
        413 => builder = HttpResponse::PayloadTooLarge(),
        429 => builder = HttpResponse::TooManyRequests(),

        // 5xx Server Errors
        500 => builder = HttpResponse::InternalServerError(),
        502 => builder = HttpResponse::BadGateway(),
        503 => builder = HttpResponse::ServiceUnavailable(),

        // Default to 200 Success
        _ => builder = HttpResponse::Ok(),
    }
    builder
}
