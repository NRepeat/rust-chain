use crate::domain::{mempool_repository::MempoolRepository, transaction::Transaction};
use std::collections::VecDeque;

pub struct Mempool<T: MempoolRepository> {
    repository: T,
}

impl<T: MempoolRepository> Mempool<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if self.check_if_transaction_valid(&transaction) {
            self.repository.add_transaction(transaction);
            println!("Transaction added.");
        } else {
            println!("Transaction invalid or already exists.");
        }
    }

    fn check_if_transaction_valid(&self, transaction: &Transaction) -> bool {
        if !transaction.is_valid() {
            return false;
        }

        if self.repository.check_exists_by_id(&transaction.id) {
            return false;
        }

        return true;
    }

    pub fn get_all_transactions(&self) -> &VecDeque<Transaction> {
        self.repository.get_all_transactions()
    }

    pub fn get_last_transaction(&self) -> Option<&Transaction> {
        self.repository.get_last_transaction()
    }

    pub fn drain_transactions(&mut self) -> VecDeque<Transaction> {
        self.repository.drain_transactions()
    }
}
