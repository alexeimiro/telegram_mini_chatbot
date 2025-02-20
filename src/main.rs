mod routes;       // Contains route definitions
mod handlers;     // Contains HTTP request handlers
mod services;     // Contains business logic and external API calls
mod models;       // Contains data models (e.g., request/response structures)
mod config;       // Handles environment variables and configuration

use std::net::SocketAddr; // For specifying the server's address
use log::info;

#[tokio::main] // Marks the main function as asynchronous
async fn main() {
    env_logger::init();
    config::load_env();

    info!("Starting server...");
    let app = routes::create_routes();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}