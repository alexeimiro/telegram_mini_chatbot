use sqlx::PgPool;
use crate::models::{IncomingMessage};
use reqwest::Client;
use crate::config;

lazy_static::lazy_static! {
    static ref DB_POOL: PgPool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect_lazy(&database_url).expect("Failed to connect to database")
    };
}

// Simulate database operations
pub async fn handle_user_login(user_id: u64, username: &str) {
    sqlx::query!(
        "INSERT INTO users (user_id, username, usage_count, is_subscribed) VALUES ($1, $2, 0, FALSE) ON CONFLICT (user_id) DO NOTHING",
        user_id,
        username
    )
    .execute(&*DB_POOL)
    .await
    .expect("Failed to insert user");
}

pub async fn get_user_usage(user_id: u64) -> u32 {
    sqlx::query_scalar!("SELECT usage_count FROM users WHERE user_id = $1", user_id)
        .fetch_one(&*DB_POOL)
        .await
        .unwrap_or(0)
}

pub async fn increment_user_usage(user_id: u64) {
    sqlx::query!("UPDATE users SET usage_count = usage_count + 1 WHERE user_id = $1", user_id)
        .execute(&*DB_POOL)
        .await
        .expect("Failed to update usage count");
}

pub async fn is_user_subscribed(user_id: u64) -> bool {
    sqlx::query_scalar!("SELECT is_subscribed FROM users WHERE user_id = $1", user_id)
        .fetch_one(&*DB_POOL)
        .await
        .unwrap_or(false)
}

pub async fn forward_message_to_telegram(
    client: &Client,
    message: IncomingMessage,
) -> Result<(), String> {
    let bot_token = config::telegram_bot_token();
    let chat_id = config::telegram_chat_id();
    let telegram_message = format!("User {}: {}", message.user_id, message.message);
    let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let params = [("chat_id", chat_id), ("text", telegram_message)];

    match client.post(&telegram_url).form(&params).send().await {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to forward message to Telegram".to_string()),
    }
}

pub async fn process_payment(user_id: u64, payment_method: &str) -> Result<(), String> {
    // Simulate payment processing
    if payment_method == "mir" || payment_method == "ton" {
        sqlx::query!("UPDATE users SET is_subscribed = TRUE WHERE user_id = $1", user_id)
            .execute(&*DB_POOL)
            .await
            .expect("Failed to update subscription status");
        Ok(())
    } else {
        Err("Invalid payment method".to_string())
    }
}