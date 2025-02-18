//src/services.rs
use crate::models::IncomingMessage; // Import the IncomingMessage model
use reqwest::Client; // HTTP client for making external API calls
use crate::config; // Import configuration utilities

// Function to forward a message to Telegram
pub async fn forward_message_to_telegram(
    client: &Client, // Shared HTTP client
    message: IncomingMessage, // Incoming message from the user
) -> Result<(), String> {
    // Retrieve the Telegram bot token from the environment variables
    let bot_token = config::telegram_bot_token();
    // Retrieve the Telegram chat ID from the environment variables
    let chat_id = config::telegram_chat_id();
    // Format the message to include the user ID and message content
    let telegram_message = format!("User {}: {}", message.user_id, message.message);

    // Construct the URL for the Telegram sendMessage API
    let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    // Define the parameters for the API request
    let params = [("chat_id", chat_id), ("text", telegram_message)];

    // Send the message to Telegram using the HTTP client
    match client.post(&telegram_url).form(&params).send().await {
        // If successful, return Ok
        Ok(_) => Ok(()),
        // If there's an error, return an error message
        Err(_) => Err("Failed to forward message to Telegram".to_string()),
    }
}