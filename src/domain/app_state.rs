use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::blockchain_repository::BlockchainRepository;
use crate::domain::mempool_repository::MempoolRepository;
use crate::domain::node::Node;
use crate::domain::user_state_repository::UserStateRepository;

pub struct AppState<B, M, U>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    pub blockchain_repo: Arc<Mutex<B>>,
    pub mempool_repo: Arc<Mutex<M>>,
    pub user_state_repo: Arc<Mutex<U>>,
    pub node: Arc<Mutex<Node>>,
    pub shared_key: String,
}

impl<B, M, U> Clone for AppState<B, M, U>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            blockchain_repo: Arc::clone(&self.blockchain_repo),
            mempool_repo: Arc::clone(&self.mempool_repo),
            user_state_repo: Arc::clone(&self.user_state_repo),
            node: Arc::clone(&self.node),
            shared_key: self.shared_key.clone(),
        }
    }
}
