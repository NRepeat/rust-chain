use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::{block::Block, blockchain_repository::BlockchainRepository};

pub async fn add_block_to_chain<B>(blockchain_repository: Arc<Mutex<B>>, block: Block)
where
    B: BlockchainRepository + Send + Sync + 'static,
{
    let mut repo_lock = blockchain_repository.lock().await;
    repo_lock.add_block(block).await;
}
