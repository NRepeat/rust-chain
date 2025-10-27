use crate::domain::transaction::Transaction;
use serde::Serialize;
use sha256::digest;

#[derive(Debug, Serialize, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        let mut block = Self {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let transactions_string = self
            .transactions
            .iter()
            .map(|t| format!("{}{}{}", t.from, t.to, t.amount))
            .collect::<Vec<String>>()
            .join("");

        let s = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, transactions_string, self.previous_hash, self.nonce
        );
        digest(s.as_bytes())
    }

    pub fn mine_block(&mut self, difficulty: u32) {
        let target = "0".repeat(difficulty as usize);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}
