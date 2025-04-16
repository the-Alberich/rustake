use dotenvy::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use std::sync::Arc;
use std::{env, time::Duration};

pub async fn init_provider() -> anyhow::Result<Arc<Provider<Http>>> {
    dotenv().ok();
    let rpc_url = env::var("ETH_PROVIDER_URL").expect("ETH_PROVIDER_URL must be set in .env");

    let retry_attempts: usize = env::var("RETRY_ATTEMPTS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60);

    let retry_delay: u64 = env::var("RETRY_DELAY_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000);

    for _ in 0..retry_attempts {
        match Provider::<Http>::try_from(rpc_url.clone()) {
            Ok(provider) => {
                let provider = provider.interval(Duration::from_millis(500));
                match provider.clone().get_chainid().await {
                    Ok(_) => return Ok(Arc::new(provider)),
                    Err(err) => {
                        tracing::warn!("Provider health check failed: {:?}", err);
                    }
                }
            }
            Err(err) => {
                tracing::warn!("Failed to build provider: {:?}", err);
            }
        }

        tokio::time::sleep(Duration::from_millis(retry_delay)).await;
    }

    Err(anyhow::anyhow!(
        "Failed to connect to provider after {} attempts.",
        retry_attempts
    ))
}
