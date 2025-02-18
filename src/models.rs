//src/models.rs
use serde::{Deserialize, Serialize}; // Serde for JSON serialization/deserialization

// Model for incoming messages from the chatbot
#[derive(Deserialize)] // Automatically deserialize JSON into this struct
pub struct IncomingMessage {
    pub user_id: String, // The ID of the user sending the message
    pub message: String, // The content of the message
}

// Model for API responses sent back to the client
#[derive(Serialize)] // Automatically serialize this struct into JSON
pub struct ApiResponse {
    pub status: String, // Status of the operation (e.g., "success" or "error")
    pub message: String, // Detailed message about the result
}