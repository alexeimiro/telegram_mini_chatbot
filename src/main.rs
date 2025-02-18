mod routes;       // Contains route definitions
mod handlers;     // Contains HTTP request handlers
mod services;     // Contains business logic and external API calls
mod models;       // Contains data models (e.g., request/response structures)
mod config;       // Handles environment variables and configuration

use std::net::SocketAddr; // For specifying the server's address

#[tokio::main] // Marks the main function as asynchronous
async fn main() {
    // Load environment variables from the `.env` file
    config::load_env();
    
    // Create the application routes using the `create_routes` function
    let app = routes::create_routes();
    
    // Define the server's address (localhost:3000)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    // Print a message to indicate the server is running
    println!("Server running on http://{}", addr);
    
    // Start the server and bind it to the specified address
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}