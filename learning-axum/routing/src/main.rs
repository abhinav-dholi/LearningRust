use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Define the router with a single route
    let app = Router::new()
        .route("/", get(handler)); // GET request at "/"

    // Define the address to run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler function
async fn handler() -> &'static str {
    "Hello, Axum!"
}