
use crate::domain::transaction::Transaction;
use std::collections::HashMap;
use uuid::Uuid;

pub trait UserStateRepository {
    fn get_balances(&self) -> &HashMap<Uuid, f64>;
    fn get_balance(&self, address: &Uuid) -> f64;
    fn apply_transaction(&mut self, transaction: &Transaction) -> bool;
}
