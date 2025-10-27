use crate::domain::transaction::Transaction;
use crate::domain::{block::Block, blockchain_repository::BlockchainRepository};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

pub async fn add_new_block<B>(
    blockchain_repository: Arc<Mutex<B>>,
    transactions: Vec<Transaction>,
    proposer_id: String,
    shared_key: String,
) where
    B: BlockchainRepository + Send + Sync + 'static,
{
    let mut repo_lock = blockchain_repository.lock().await;

    if let Some(last_block) = repo_lock.get_last_block().await {
        let new_block = Block::new(
            last_block.index + 1,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            proposer_id.to_string(),
            last_block.header.height + 1,
            transactions,
            last_block.hash.clone(),
            shared_key,
        );

        repo_lock.add_block(new_block).await;
    }
}
