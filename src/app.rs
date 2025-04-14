use axum::{routing::get, Router};

async fn health_check() -> &'static str {
    "OK"
}

pub async fn build_app() -> anyhow::Result<Router> {
    let app = Router::new().route("/health", get(health_check));

    Ok(app)
}
