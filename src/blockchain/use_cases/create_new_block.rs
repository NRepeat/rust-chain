use crate::domain::transaction::Transaction;
use crate::domain::{block::Block, blockchain_repository::BlockchainRepository};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

pub async fn create_new_block<B>(
    blockchain_repository: Arc<Mutex<B>>,
    transactions: Vec<Transaction>,
    proposer_id: &str,
    shared_key: &str,
) -> Block
where
    B: BlockchainRepository + Send + Sync + 'static,
{
    let repo_lock = blockchain_repository.lock().await;
    let last_block = repo_lock.get_last_block().await;

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
        shared_key.to_string(),
    );

    new_block
}
