use axum::{
    extract::{Path, State},
    Json,
};
use ethers::types::H160;
use std::str::FromStr;

// Rustake
use crate::{contracts::staking_pool, error::AppError, state::AppState};

pub async fn get_rewards_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<String>, AppError> {
    let address = H160::from_str(&address).map_err(|e| {
        AppError::InvalidAddressFormat(format!("Not a valid Ethereum address: {e:?}"))
    })?;

    let rewards = staking_pool::get_rewards(&state.cached_contract, address)
        .await
        .map_err(|e| AppError::EvmCallFailed(e.to_string()))?;

    Ok(Json(format!("{rewards}")))
}
