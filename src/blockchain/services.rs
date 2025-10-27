use std::collections::VecDeque;

use uuid::Uuid;

use crate::domain::{block::Block, transaction::Transaction};

pub trait BlockchainRepository {
    fn get_all_blocks(&self) -> Vec<Block>;
    fn add_block(&mut self, block: Block);
    fn get_last_block(&self) -> Option<Block>;
}

pub trait MempoolRepsoitory {
    fn add_transaction(&mut self, transaction: Transaction);
    fn get_all_transactions(&self) -> VecDeque<Transaction>;
    fn get_last_transactions(&self) -> Transaction;
    fn check_exists_by_id(&self, transaction_id: &Uuid) -> bool;
}
