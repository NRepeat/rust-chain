use std::env;

use crate::domain::block::Block;
use crate::domain::blockchain_repository::BlockchainRepository;

pub fn create_genesis_block(blockchain_repository: &mut dyn BlockchainRepository) {
    let shared_key = env::var("SHARED_KEY").expect("SHARED_KEY");

    let genesis_block = Block::new(
        0,
        0,
        "GENESIS".to_string(),
        0,
        vec![],
        "0".to_string(),
        shared_key,
    );

    blockchain_repository.add_block(genesis_block);
}
