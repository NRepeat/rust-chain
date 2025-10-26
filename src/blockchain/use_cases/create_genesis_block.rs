use crate::blockchain::services::BlockchainRepository;
use crate::domain::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_genesis_block(blockchain_repository: &mut dyn BlockchainRepository, difficulty: u32) {
    let mut genesis_block = Block::new(
        0,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        vec![],
        "0".to_string(),
    );
    genesis_block.mine_block(difficulty);
    blockchain_repository.add_block(genesis_block);
}
