/*
* Middleware Guard, a module that provides a security layer to check the request source before handle it:
*  —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— ——
* | 1. Get the request path                                                                             |
* | 2. Check if the path is public or private                                                           |
* | 3. If the path is public                                                                            |
* | 4. Get the token from the request `Authorization` header                                            |
* | 6. Handle the `_tokens_payload.access_token` and `_tokens_payload.refresh_token`                    |
* | 7. Call the next Function or Reject the request                                                     |
*  —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— —— ——
* This module is the security layer that filters the server-app from any Unauthorized Requests
*/

use crate::structs::user::User;
use crate::{
    constants::{responder::global_auth_guard_right_body, routes::PUBLIC_ROUTES},
    structs::{
        jwt::{JwtManager, TokensPayload},
        middleware::MiddlewareExtensionsData,
        response::{GlobalResponse, NextResultResponder},
    },
};
use actix_web::{body::MessageBody, dev::ServiceRequest, middleware::Next, HttpMessage};

pub async fn auth_guard(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> NextResultResponder {
    // 1. Get the request path
    let path = req.path();

    // 2. Check if the path is public or private
    let is_public = PUBLIC_ROUTES
        .iter()
        .any(|route_regex| route_regex.is_match(path));

    // 3. If the path is public, we ignore the authentication check and continue
    // to the next function, otherwise we need to check the request source first.
    if is_public {
        return call_next_function(next, req).await;
    }

    // 4. Get the token from the request `Authorization` header.
    // If not exists, we should return a `401 Unauthorized` response.
    if let Some(token) = get_token_from_header(&req) {
        let jwt_manager = JwtManager::new().unwrap();

        // 5. Decode the token and extract the `TokensPayload`
        let decoded_tokens = jwt_manager.decode_jwt_and_decrypt(token);
        println!("decoded_tokens -> {:?}", decoded_tokens);

        match decoded_tokens {
            Ok(token) => {
                let user_payload = serde_json::from_str::<User>(&token);
                match user_payload {
                    Ok(user) => {
                        // 6. Handle the `tokens_payload.access_token` and `tokens_payload.refresh_token` depend on your situation
                        // best practice is to check the user from your authentication provider such as `Firebase Authentication`, `AWS Cognito`, `Azure AD B2C`, etc.
                        // NOTE: Also it's safe to store the user ID or user EMAIL in the token payload as we always encrypt the payload before send it to the user client, so you can extract them here wihtout the double check of the authenticated user.

                        // 7. If the user is found and authenticated with the `tokens_payload.accessToken`, you can continue to the next function.
                        // Also, inject the `MiddlewareExtensionsData` to the request extensions to use it later in the next function.

                        // For now, I'll add this standard unique user id `790505571683` as a placeholder.
                        let extracted_user_id = String::from("790505571683");

                        req.extensions_mut().insert(MiddlewareExtensionsData {
                            user_id: user.id,
                            // The refresh token always None in this case. We expect only the access token,
                            // which comes with the request via Authorization header, but you can add it either and extract it
                            // to rotate the token and ignore multiple tokens generation, but for now it is optional only.
                            // I'll implement it in the future updates.
                            refresh_token: None,
                        });

                        call_next_function(next, req).await
                    }
                    // Note: In this case, this error comes from the app itself, not from the user,
                    // so we will return a `500 Internal Server Error` response.
                    Err(e) => {
                        eprintln!("Error Parse Payloads -> {:?}", e);
                        global_auth_guard_right_body(
                            req,
                            500,
                            &GlobalResponse::<serde_json::Value> {
                                message: Some(String::from("internal-error")),
                                data: None,
                            },
                        )
                    }
                }
            }
            Err(e) => global_auth_guard_right_body(
                req,
                401,
                &GlobalResponse::<serde_json::Value> {
                    message: Some(String::from("unauthorized-request")),
                    data: None,
                },
            ),
        }
    } else {
        global_auth_guard_right_body(
            req,
            401,
            &GlobalResponse::<serde_json::Value> {
                message: Some(String::from("unauthorized-request")),
                data: None,
            },
        )
    }
}

async fn call_next_function(
    next: Next<impl MessageBody + 'static>,
    req: ServiceRequest,
) -> NextResultResponder {
    let res = next.call(req).await?;
    Ok(res.map_into_boxed_body())
}

/// Extract the token from the request `Authorization` header, it should be `Bearer <token>`.
/// Return `Some(token)` or `None` if no token is found
fn get_token_from_header(req: &ServiceRequest) -> Option<&str> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token);
            }
        }
    }
    None
}
