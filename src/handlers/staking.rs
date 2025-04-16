use axum::extract::{Json, State};
use ethers::types::U256;
use serde::Deserialize;

// Rustake
use crate::{
    auth::extractors::SignerKey,
    contracts::staking_pool::{stake, unstake},
    error::AppError,
    state::AppState,
};

#[derive(Deserialize)]
pub struct StakeRequest {
    pub amount: String,
    pub lock_duration: String,
}

pub async fn stake_handler(
    State(state): State<AppState>,
    SignerKey(signer): SignerKey,
    Json(payload): Json<StakeRequest>,
) -> Result<Json<String>, AppError> {
    let amount =
        U256::from_dec_str(&payload.amount).map_err(|_| AppError::BadRequest("Invalid amount."))?;
    let lock_duration = U256::from_dec_str(&payload.lock_duration)
        .map_err(|_| AppError::BadRequest("Invalid lock duration."))?;

    let tx_hash = stake(signer, state.provider.clone(), amount, lock_duration)
        .await
        .map_err(|e| AppError::Internal(format!("Staking failed. {}", e)))?;

    Ok(Json(format!("{tx_hash}")))
}

pub async fn unstake_handler(
    State(state): State<AppState>,
    SignerKey(signer): SignerKey,
) -> Result<Json<String>, AppError> {
    let tx_hash = unstake(signer, state.provider.clone())
        .await
        .map_err(|e| AppError::Internal(format!("Staking failed. {}", e)))?;

    Ok(Json(format!("{tx_hash}")))
}
