use std::collections::VecDeque;

use uuid::Uuid;

use crate::domain::transaction::Transaction;

pub trait MempoolRepository {
    fn add_transaction(&mut self, transaction: Transaction);
    fn get_all_transactions(&self) -> &VecDeque<Transaction>;
    fn get_last_transaction(&self) -> Option<&Transaction>;
    fn check_exists_by_id(&self, transaction_id: &Uuid) -> bool;
    fn drain_transactions(&mut self) -> VecDeque<Transaction>;
}
