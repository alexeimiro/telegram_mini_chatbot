use axum::{Router, routing::{post, get}};
use crate::handlers;
use reqwest::Client;

pub fn create_routes() -> Router {
    let http_client = Client::new();

    Router::new()
        .route("/", get(|| async { "Welcome to the Chatbot Backend!" }))
        .route("/auth/telegram", post(handlers::auth_telegram))
        .route("/message", post(handlers::receive_message))
        .route("/subscribe", post(handlers::subscribe_user))
        .with_state(http_client)
}