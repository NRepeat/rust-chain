use crate::domain::{block::Block, transaction::Transaction};
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

#[async_trait]
pub trait UserStateRepository: Send + Sync {
    fn get_balances(&self) -> &HashMap<Uuid, f64>;
    fn get_balance(&self, address: &Uuid) -> f64;
    fn set_balance(&mut self, address: Uuid, balance: f64);
    fn apply_transaction(&mut self, transaction: &Transaction) -> bool;
    async fn rebuild_from_blocks(&mut self, blocks: &Vec<Block>);
}
