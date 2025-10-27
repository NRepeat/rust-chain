use crate::domain::{block::Block, blockchain_repository::BlockchainRepository};

pub struct InMemoryBlockchainRepository {
    blocks: Vec<Block>,
}

impl InMemoryBlockchainRepository {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }
}

impl BlockchainRepository for InMemoryBlockchainRepository {
    fn get_all_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn get_last_block(&self) -> Option<Block> {
        self.blocks.last().cloned()
    }
}
