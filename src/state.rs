use ethers::contract::Contract;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use k256::ecdsa::SigningKey;
use std::sync::Arc;
use tokio::sync::RwLock;

// Signer type
pub type WalletSigner = Wallet<SigningKey>;

// Define app-wide state
#[derive(Clone)]
pub struct AppState {
    pub provider: Arc<Provider<Http>>,
    pub cached_contract: CachedContract,
}

// Wrapping the state contract inside a thread-safe async RwLock.
// This is more complex than the Provider, since the Provider is read-only after init.
// The cached contract is meant to be cached, but may be hot-reloaded if the contract
// ABI is updated while RUSTAKE is running (planned work, hot-reload is not yet suppported).
#[derive(Clone)]
pub struct CachedContract {
    inner: Arc<RwLock<Option<Contract<SignerMiddleware<Arc<Provider<Http>>, WalletSigner>>>>>,
}

impl CachedContract {
    pub fn new() -> Self {
        CachedContract {
            inner: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get(
        &self,
    ) -> Option<Contract<SignerMiddleware<Arc<Provider<Http>>, WalletSigner>>> {
        self.inner.read().await.clone()
    }

    pub async fn set(
        &self,
        contract: Contract<SignerMiddleware<Arc<Provider<Http>>, WalletSigner>>,
    ) {
        let mut write_guard = self.inner.write().await;
        *write_guard = Some(contract);
    }
}
