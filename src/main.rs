// External dependencies
use axum::{routing::{get, post}, Router};
use ethers::signers::Signer;
use owo_colors::OwoColorize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};

// Rustake
mod auth;
mod contracts;
mod error;
mod handlers;
mod provider;
mod state;
mod tracing_config;

use crate::state::{AppState, CachedContract, WalletSigner};

#[tokio::main]
async fn main() {
    // Load .env before reading any env vars.
    dotenvy::dotenv().ok();

    // ASCII flair, because of course.
    if std::env::var("RUSTAKE_DISABLE_ASCII").is_err() {
        let ascii_header = include_str!("./assets/ascii_header.txt");
        println!("{}", ascii_header.truecolor(255, 165, 0).bold());
    }

    // Optional TODO details.
    if std::env::var("RUSTAKE_TODO").is_ok() {
        let todo = include_str!("../TODO.md");
        println!("\n{}", todo);
    }

    // Initialize tracing.
    tracing_config::init();

    // Create a provider.
    let provider = match provider::init_provider().await {
        Ok(p) => p,
        Err(err) => {
            error!("!!Fatal!! Failed to initialize provider: {:?}", err);
            std::process::exit(1);
        }
    };

    // Create the static contract.
    let cached_contract = CachedContract::new();
    if let Ok(signer_key) = std::env::var("DEFAULT_SIGNER_KEY") {
        if let Ok(wallet) = signer_key.parse::<WalletSigner>() {
            contracts::staking_pool::init_contract(
                wallet.with_chain_id(1337u64),
                provider.clone(),
                &cached_contract
            ).await;
        }
    }

    // Initialize the app state.
    let app_state = AppState {
        provider: provider.clone(),
        cached_contract: cached_contract.clone(),
    };

    // Define the router.
    let app = Router::new()
        .route("/", get(|| async { "Hello from RUSTAKE." }))
        .route("/health", get(handlers::health::health_check))
        .route("/rewards/:address", get(handlers::rewards::get_rewards_handler))
        .route("/stake", post(handlers::staking::stake_handler))
        .route("/unstake", post(handlers::staking::unstake_handler))
        .with_state(app_state);

    // Bind and serve.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server Address: http://{}", addr);
    axum::serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
