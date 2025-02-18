//src/handlers.rs
use axum::{
    extract::State, // Extracts shared state (e.g., HTTP client)
    Json, // Extracts and serializes JSON payloads
};
use crate::models::{IncomingMessage, ApiResponse}; // Import data models
use crate::services; // Import business logic and external API calls
use reqwest::Client; // HTTP client for making external API calls

// Handler for the "/message" route
pub async fn receive_message(
    State(client): State<Client>, // Extract the shared HTTP client
    Json(payload): Json<IncomingMessage>, // Extract and deserialize the JSON payload
) -> Json<ApiResponse> {
    // Call the service to forward the message to Telegram
    match services::forward_message_to_telegram(&client, payload).await {
        // If successful, return a success response
        Ok(_) => Json(ApiResponse {
            status: "success".to_string(),
            message: "Message forwarded to Telegram".to_string(),
        }),
        // If there's an error, return an error response
        Err(err) => Json(ApiResponse {
            status: "error".to_string(),
            message: err,
        }),
    }
}