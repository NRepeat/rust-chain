use std::env;

use crate::domain::{
    block::Block, blockchain_repository::BlockchainRepository, transaction::Transaction,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
pub async fn create_genesis_block<B>(blockchain_repository: Arc<Mutex<B>>)
where
    B: BlockchainRepository + Send + Sync + 'static,
{
    let shared_key = env::var("SHARED_KEY").expect("SHARED_KEY");

    let genesis_sender_str = env::var("GENESIS_SENDER_ID").expect("GENESIS_SENDER_ID");
    let faucet_wallet_str = env::var("FAUCET_WALLET_ID").expect("FAUCET_WALLET_ID");

    let genesis_sender_id = Uuid::parse_str(&genesis_sender_str).unwrap();
    let faucet_wallet_id = Uuid::parse_str(&faucet_wallet_str).unwrap();

    let genesis_tx = Transaction {
        id: Uuid::new_v4(),
        from: genesis_sender_id, // <-- ОТ "СИСТЕМЫ"
        to: faucet_wallet_id,    // <-- НА "КРАН"
        amount: 1000000.0,
        timestamp: 0,
    };

    let mut repo_lock = blockchain_repository.lock().await;

    let genesis_block = Block::new(
        0,
        0,
        "GENESIS".to_string(),
        0,
        vec![genesis_tx],
        "0".to_string(),
        shared_key.to_string(),
    );

    repo_lock.add_block(genesis_block.clone()).await;
}
