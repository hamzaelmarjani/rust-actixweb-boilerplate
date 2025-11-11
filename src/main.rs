use crate::utils::logging::print_welcome_message;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, middleware::from_fn, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

mod constants;
mod controllers;
mod jwt;
mod mails;
mod middleware;
mod structs;
mod test;
mod utils;
mod views;

#[get("/health")]
/// An entry point to check the app's health.
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server App is running successfully!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Load the Environment Variables from the `.env ` file, or from env variables
    dotenv().ok();

    // 2. First, call all the initializers, like: databases url, redis, aws, gcp, azure, etc.

    print_welcome_message();

    // 3. Set up the server, read more about ActixWeb HttpServer at: `https://actix.rs/docs/server`
    HttpServer::new(|| {
        App::new()
            // =======================================================
            // • Console Logger Configurations
            // =======================================================
            // Logger: for pretty console log
            .wrap(Logger::default())
            // =======================================================
            // • Middleware Integrations
            // =======================================================
            // Middleware integrations
            // 1. Auth Guard
            .wrap(from_fn(middleware::guard::auth_guard))
            // =======================================================
            // • App Cors Configurations
            // ========================================================
            // Add only the cors accepted by your server & client side
            .wrap(
                Cors::default()
                    // The client development URL (change the port `4321` to yours)
                    .allowed_origin("http://localhost:4321")
                    // The accepted domains (example.com, *.example.com, etc)
                    .allowed_origin("https://example.com")
                    .allowed_origin("https://*.example.com")
                    // Allowed Http requests methods ("GET", "POST, "PUT", "DELETE", "PATCH")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                    // Allowed headers from the client
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    // This Injects the `Access-Control-Allow-Credentials` header in responses
                    .supports_credentials()
                    // Maximum timeout response in seconds (240 secs = 4 mins)
                    .max_age(240),
            )
            // =======================================================
            // • Check the app's health.
            .service(health)
            // =======================================================
            // • App Services and Entry Points
            // =======================================================
            // 1. Crud Entry Points
            // 1.1 Create Data
            .service(controllers::crud::create::create_data)
            // 1.2 Read Data
            .service(controllers::crud::read::read_data)
            // 1.3 Update Data
            .service(controllers::crud::update::update_data)
            // 1.4 Delete Data
            .service(controllers::crud::delete::delete_data)
            // =======================================================
            // 2. User
            // 2.1 User Lifecycle
            .service(controllers::user::lifecycle::user_lifecycle)
            // 2.2 User Sign-in: -> Public Entry Point
            .service(controllers::user::sign_in::user_sign_in)
            // 2.3 User Sign-up: -> Public Entry Point
            .service(controllers::user::sign_up::user_sign_up)
            // 2.4 User Email Verification: -> Public Entry Point
            .service(controllers::user::email_verification::user_email_verification)
    })
    // 4. Server runs on all those urls:
    // `http://localhost:8080` or `http://127.0.0.1:8080` or `http://0.0.0.0:8080`
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
