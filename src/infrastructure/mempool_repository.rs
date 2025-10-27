use std::collections::VecDeque;

use uuid::Uuid;

use crate::domain::{mempool_repository::MempoolRepository, transaction::Transaction};

pub struct InMemoryMempoolRepository {
    pub transactions: VecDeque<Transaction>,
}

impl InMemoryMempoolRepository {
    pub fn new() -> Self {
        Self {
            transactions: VecDeque::new(),
        }
    }
}

impl MempoolRepository for InMemoryMempoolRepository {
    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push_back(transaction);
    }
    fn get_all_transactions(&self) -> &VecDeque<Transaction> {
        &self.transactions
    }

    fn check_exists_by_id(&self, transaction_id: &Uuid) -> bool {
        self.transactions.iter().any(|t| t.id == *transaction_id)
    }
    fn drain_transactions(&mut self) -> VecDeque<Transaction> {
        self.transactions.drain(..).collect()
    }
}
