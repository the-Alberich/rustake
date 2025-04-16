use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use ethers::signers::{LocalWallet, Signer};
use std::{env, str::FromStr};

// Rustake
use crate::error::AppError;

pub struct SignerKey(pub LocalWallet);

#[async_trait]
impl<S> FromRequestParts<S> for SignerKey
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let key = parts
            .headers
            .get("x-signer-key")
            .ok_or_else(|| AppError::SignerExtractionError("Missing signer key header."))?
            .to_str()
            .map_err(|_| AppError::SignerExtractionError("Invalid signer key header format."))?;

        let chain_id = env::var("CHAIN_ID")
            .unwrap_or_else(|_| "31337".into())
            .parse::<u64>()
            .unwrap_or(31337);

        let wallet = LocalWallet::from_str(key)
            .map_err(|_| AppError::Unauthorized(format!("Invalid signer key. {}", key)))?
            .with_chain_id(chain_id);

        Ok(SignerKey(wallet))
    }
}
