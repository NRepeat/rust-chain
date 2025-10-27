use crate::domain::block::Block;
use async_trait::async_trait;

#[async_trait]
pub trait BlockchainRepository: Send + Sync {
    async fn get_all_blocks(&self) -> Vec<Block>;
    async fn add_block(&mut self, block: Block);
    async fn get_last_block(&self) -> Option<Block>;
}
