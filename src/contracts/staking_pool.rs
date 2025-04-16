use dotenvy::dotenv;
use ethers::{abi::Abi, prelude::*};
use std::{env, str::FromStr, sync::Arc};

// Rustake
use crate::error::AppError;
use crate::state::{CachedContract, WalletSigner};

// Contract address from .env
static CONTRACT_ADDRESS: Lazy<Address> = Lazy::new(|| {
    dotenv().ok();
    let addr_str = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set in .env file");
    Address::from_str(&addr_str).expect("Invalid CONTRACT_ADDRESS format.")
});

// ABI from file
static ABI_BYTES: &[u8] =
    include_bytes!("../import/artifacts/contracts/StakingPool.sol/StakingPool.json");

// Contract ABI
static CONTRACT_ABI: Lazy<Abi> =
    Lazy::new(|| serde_json::from_slice(ABI_BYTES).expect("Failed to parse ABI from JSON."));

// Initialize the contract and cache it via shared app state
pub async fn init_contract(
    signer: WalletSigner,
    provider: Arc<Provider<Http>>,
    cached: &CachedContract,
) {
    let client = Arc::new(SignerMiddleware::new(provider.clone(), signer));
    let contract = Contract::new(*CONTRACT_ADDRESS, CONTRACT_ABI.clone(), client);
    cached.set(contract).await;
}

// Build contract with dynamic signer (for user-authenticated write calls)
fn build_contract<S: Signer + Clone + 'static>(
    signer: S,
    provider: Arc<Provider<Http>>,
) -> Contract<SignerMiddleware<Arc<Provider<Http>>, S>> {
    let client = Arc::new(SignerMiddleware::new(provider.clone(), signer));
    Contract::new(*CONTRACT_ADDRESS, CONTRACT_ABI.clone(), client)
}

// View call: calculateRewards(address)
pub async fn get_rewards(
    cached_contract: &CachedContract,
    address: H160,
) -> Result<U256, Box<dyn std::error::Error>> {
    let contract = cached_contract
        .get()
        .await
        .ok_or("Contract not initialized")?;
    let reward: U256 = contract.method("calculateRewards", address)?.call().await?;
    Ok(reward)
}

// Write call: stake(amount, lockDuration)
pub async fn stake<S: Signer + Clone + 'static>(
    signer: S,
    provider: Arc<Provider<Http>>,
    amount: U256,
    lock_duration: U256,
) -> Result<TxHash, Box<dyn std::error::Error>> {
    let contract = build_contract(signer, provider);
    let method = contract
        .method::<(U256, U256), H256>("stake", (amount, lock_duration))
        .map_err(|e| AppError::EvmCallFailed(format!("Stake method error: {e:?}")))?;
    let tx = method.send().await?;
    Ok(tx.tx_hash())
}

// Write call: unstake()
pub async fn unstake<S: Signer + Clone + 'static>(
    signer: S,
    provider: Arc<Provider<Http>>,
) -> Result<TxHash, Box<dyn std::error::Error>> {
    let contract = build_contract(signer, provider);
    let method = contract
        .method::<(), H256>("unstake", ())
        .map_err(|e| AppError::EvmCallFailed(format!("Unstake method error: {e:?}")))?;
    let tx = method.send().await?;
    Ok(tx.tx_hash())
}
