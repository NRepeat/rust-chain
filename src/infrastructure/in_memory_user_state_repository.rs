use crate::domain::block::Block;
use crate::domain::transaction::Transaction;
use crate::domain::user_state_repository::UserStateRepository;
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

pub struct InMemoryUserStateRepository {
    balances: HashMap<Uuid, f64>,
}

impl InMemoryUserStateRepository {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }
}
#[async_trait]
impl UserStateRepository for InMemoryUserStateRepository {
    fn get_balances(&self) -> &HashMap<Uuid, f64> {
        &self.balances
    }

    fn get_balance(&self, address: &Uuid) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }

    fn set_balance(&mut self, address: Uuid, balance: f64) {
        self.balances.insert(address, balance);
    }

    fn apply_transaction(&mut self, transaction: &Transaction) -> bool {
        let sender_balance = self.get_balance(&transaction.from);
        println!("Sender balance: {}", sender_balance);
        println!("Transaction amount: {}", transaction.amount);
        if sender_balance < transaction.amount {
            println!("Insufficient balance for sender {}", transaction.from);
            return false;
        }

        let new_sender_balance = sender_balance - transaction.amount;
        self.balances.insert(transaction.from, new_sender_balance);

        let receiver_balance = self.get_balance(&transaction.to);
        let new_receiver_balance = receiver_balance + transaction.amount;
        self.balances.insert(transaction.to, new_receiver_balance);

        println!(
            "Transaction applied: {} -> {}",
            transaction.from, transaction.to
        );
        true
    }
    async fn rebuild_from_blocks(&mut self, blocks: &Vec<Block>) {
        self.balances.clear();

        for block in blocks.iter() {
            for tx in &block.transactions {
                let sender_balance = self.get_balance(&tx.from);
                let new_sender_balance = sender_balance - tx.amount;
                self.balances.insert(tx.from, new_sender_balance);

                let receiver_balance = self.get_balance(&tx.to);
                let new_receiver_balance = receiver_balance + tx.amount;
                self.balances.insert(tx.to, new_receiver_balance);
            }
        }
    }
}
