use crate::domain::transaction::Transaction;
use crate::domain::{block::Block, blockchain_repository::BlockchainRepository};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn add_new_block(
    blockchain_repository: &mut dyn BlockchainRepository,
    transactions: Vec<Transaction>,
    difficulty: u32,
) {
    let last_block = blockchain_repository.get_last_block().unwrap();
    let new_block = Block::new(
        last_block.index + 1,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        transactions,
        last_block.hash.clone(),
    );
    let mut new_block_mined = new_block.clone();
    new_block_mined.mine_block(difficulty);
    blockchain_repository.add_block(new_block_mined);
}
