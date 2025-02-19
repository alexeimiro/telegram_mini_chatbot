use axum::{
    extract::State, // Extracts shared state (e.g., HTTP client)
    Json, // Extracts and serializes JSON payloads
};
use crate::models::{IncomingMessage, ApiResponse, AuthRequest, SubscriptionRequest}; // Import data models
use crate::services; // Import business logic and external API calls
use reqwest::Client; // HTTP client for making external API calls
use sha2::{Sha256, Digest};
use hex;

// Handler for the "/auth/telegram" route
pub async fn auth_telegram(
    State(client): State<Client>,
    Json(payload): Json<AuthRequest>,
) -> Json<ApiResponse> {
    match verify_init_data(&payload.initData) {
        Ok(true) => {
            let user_data = url::form_urlencoded::parse(payload.initData.as_bytes())
                .filter(|(key, _)| key == "user")
                .map(|(_, value)| value)
                .next()
                .and_then(|user| serde_json::from_str::<serde_json::Value>(&user).ok());

            if let Some(user) = user_data {
                let user_id = user["id"].as_u64().unwrap_or(0);
                let username = user["username"].as_str().unwrap_or("");

                services::handle_user_login(user_id, username).await;

                Json(ApiResponse {
                    status: "success".to_string(),
                    message: "User authenticated".to_string(),
                })
            } else {
                Json(ApiResponse {
                    status: "error".to_string(),
                    message: "Invalid user data".to_string(),
                })
            }
        }
        _ => Json(ApiResponse {
            status: "error".to_string(),
            message: "Authentication failed".to_string(),
        }),
    }
}

// Handler for the "/message" route
pub async fn receive_message(
    State(client): State<Client>,
    Json(payload): Json<IncomingMessage>,
) -> Json<ApiResponse> {
    let user_id = payload.user_id.parse::<u64>().unwrap_or(0);

    // Check if the user has exceeded the free trial limit
    let usage_count = services::get_user_usage(user_id).await;

    if usage_count >= 10 && !services::is_user_subscribed(user_id).await {
        return Json(ApiResponse {
            status: "error".to_string(),
            message: "Free trial limit reached. Please subscribe to continue.".to_string(),
        });
    }

    // Forward message to Telegram
    match services::forward_message_to_telegram(&client, payload).await {
        Ok(_) => {
            services::increment_user_usage(user_id).await;
            Json(ApiResponse {
                status: "success".to_string(),
                message: "Message forwarded to Telegram".to_string(),
            })
        }
        Err(err) => Json(ApiResponse {
            status: "error".to_string(),
            message: err,
        }),
    }
}

// Handler for the "/subscribe" route
pub async fn subscribe_user(
    Json(payload): Json<SubscriptionRequest>,
) -> Json<ApiResponse> {
    let user_id = payload.user_id.parse::<u64>().unwrap_or(0);

    match services::process_payment(user_id, &payload.payment_method).await {
        Ok(_) => Json(ApiResponse {
            status: "success".to_string(),
            message: "Subscription successful".to_string(),
        }),
        Err(err) => Json(ApiResponse {
            status: "error".to_string(),
            message: err,
        }),
    }
}

// Function to verify initData
fn verify_init_data(init_data: &str) -> Result<bool, String> {
    let mut params: Vec<(&str, &str)> = init_data
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            Some((parts.next().unwrap(), parts.next().unwrap()))
        })
        .collect();

    // Remove the "hash" parameter
    let hash = params.iter().find(|(key, _)| *key == "hash").map(|(_, value)| *value);
    params.retain(|(key, _)| *key != "hash");

    // Sort parameters by key
    params.sort_by(|a, b| a.0.cmp(b.0));

    // Concatenate sorted parameters
    let data_check_string = params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("\n");

    // Calculate SHA-256 hash
    let bot_token = crate::config::telegram_bot_token();
    let secret_key = Sha256::digest(bot_token.as_bytes());
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    hasher.update(data_check_string.as_bytes());
    let calculated_hash = hex::encode(hasher.finalize());

    // Compare hashes
    match hash {
        Some(h) if h == calculated_hash => Ok(true),
        _ => Err("Invalid initData".to_string()),
    }
}