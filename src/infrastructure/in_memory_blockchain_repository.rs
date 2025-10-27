use crate::domain::{block::Block, blockchain_repository::BlockchainRepository};
use async_trait::async_trait;

pub struct InMemoryBlockchainRepository {
    blocks: Vec<Block>,
}

impl InMemoryBlockchainRepository {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }
}

#[async_trait]
impl BlockchainRepository for InMemoryBlockchainRepository {
    async fn get_all_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    async fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    async fn get_last_block(&self) -> Block {
        self.blocks.last().cloned().unwrap()
    }
    async fn replace_chain(&mut self, new_chain: Vec<Block>) {
        self.blocks = new_chain;
    }
}
