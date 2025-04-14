// 🚀
use axum::{routing::get, Router};
use owo_colors::OwoColorize;
use std::net::SocketAddr;
use tracing::info;

mod tracing_config;

#[tokio::main]
async fn main() {
    // Load .env before reading any env vars.
    dotenvy::dotenv().ok();

    // ASCII flair, because of course.
    if !std::env::var("RUSTAKE_DISABLE_ASCII").is_ok() {
        let ascii_header = include_str!("./assets/ascii_header.txt"); // adjust path if needed
        println!("{}", ascii_header.truecolor(255, 165, 0).bold());
    }

    // Optional TODO details.
    if std::env::var("RUSTAKE_TODO").is_ok() {
        let todo = include_str!("../TODO.md");
        println!("\n{}", todo);
    }


    // Initialize tracing.
    tracing_config::init();

    // Define the router.
    let app = Router::new().route("/", get(|| async { "Hello from rustake 👋" }));

    // Bind address.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server running at http://{}", addr);

    // Run the server using axum's serve.
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
