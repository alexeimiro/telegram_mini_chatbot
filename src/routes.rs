//src/routes.rs
use axum::{Router, routing::post}; // Axum's Router and routing utilities
use crate::handlers; // Import handlers for HTTP requests
use reqwest::Client; // HTTP client for making external API calls

// Function to create and return the application routes
pub fn create_routes() -> Router {
    // Initialize an HTTP client (shared across the application)
    let http_client = Client::new();

    // Define the application routes
    Router::new()
        // Root route ("/") - Returns a welcome message
        .route("/", axum::routing::get(|| async { "Welcome to the Chatbot Backend!" }))
        // "/message" route - Handles POST requests for incoming messages
        .route("/message", post(handlers::receive_message))
        // Attach the HTTP client as shared state for all routes
        .with_state(http_client)
}