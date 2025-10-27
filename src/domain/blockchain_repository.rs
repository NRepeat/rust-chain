use crate::domain::block::Block;

pub trait BlockchainRepository {
    fn get_all_blocks(&self) -> Vec<Block>;
    fn add_block(&mut self, block: Block);
    fn get_last_block(&self) -> Option<Block>;
}
