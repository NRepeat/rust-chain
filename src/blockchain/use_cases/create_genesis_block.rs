use std::env;

use crate::domain::{block::Block, blockchain_repository::BlockchainRepository};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_genesis_block<B>(blockchain_repository: Arc<Mutex<B>>)
where
    B: BlockchainRepository + Send + Sync + 'static,
{
    let shared_key = env::var("SHARED_KEY").expect("SHARED_KEY");
    let mut repo_lock = blockchain_repository.lock().await;
    let genesis_block = Block::new(
        0,
        0,
        "GENESIS".to_string(),
        0,
        vec![],
        "0".to_string(),
        shared_key,
    );

    repo_lock.add_block(genesis_block.clone()).await;
}
