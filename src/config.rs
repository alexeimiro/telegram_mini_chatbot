//src/config.rs
use dotenv::dotenv; // Load environment variables from a `.env` file
use std::env; // Access environment variables

// Function to load environment variables from the `.env` file
pub fn load_env() {
    dotenv().ok(); // Load the `.env` file if it exists
}

// Function to retrieve the Telegram bot token from environment variables
pub fn telegram_bot_token() -> String {
    env::var("TELEGRAM_BOT_TOKEN") // Get the value of TELEGRAM_BOT_TOKEN
        .expect("TELEGRAM_BOT_TOKEN must be set") // Panic if the variable is missing
}

// Function to retrieve the Telegram chat ID from environment variables
pub fn telegram_chat_id() -> String {
    env::var("TELEGRAM_CHAT_ID") // Get the value of TELEGRAM_CHAT_ID
        .expect("TELEGRAM_CHAT_ID must be set") // Panic if the variable is missing
}