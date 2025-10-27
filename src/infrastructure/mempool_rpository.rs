use std::collections::VecDeque;

use uuid::Uuid;

use crate::{blockchain::services::MempoolRepsoitory, domain::transaction::Transaction};

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

impl MempoolRepsoitory for InMemoryMempoolRepository {
    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push_back(transaction);
    }
    fn get_all_transactions(&self) -> VecDeque<Transaction> {
        return self.transactions.clone();
    }
    fn get_last_transactions(&self) -> Transaction {
        return self.transactions.back().expect("Mempool is empty").clone();
    }
    fn check_exists_by_id(&self, transaction_id: &Uuid) -> bool {
        self.transactions.iter().any(|t| t.id == *transaction_id)
    }
}
