mod routes;       // Contains route definitions
mod handlers;     // Contains HTTP request handlers
mod services;     // Contains business logic and external API calls
mod models;       // Contains data models (e.g., request/response structures)
mod config;       // Handles environment variables and configuration

use std::net::SocketAddr; // For specifying the server's address
use log::info;
use tower_http::cors::{CorsLayer, AllowOrigin}; // Import CORS middleware
use axum::Router;

#[tokio::main] // Marks the main function as asynchronous
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Load environment variables
    config::load_env();

    info!("Starting server...");

    // Retrieve the frontend URL from the environment variable
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string()); // Default to localhost for development

    // Create the router with your defined routes
    let app = routes::create_routes();

    // Add CORS middleware to allow requests from the specified origin
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(frontend_url.parse().unwrap())) // Allow only the specified origin
        .allow_methods(Any) // Allow all HTTP methods
        .allow_headers(Any); // Allow all headers

    // Wrap the router with the CORS middleware
    let app = Router::new().nest("/", app).layer(cors);

    // Specify the server's address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    // Start the server
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}