use crate::domain::{block_header::BlockHeader, transaction::Transaction};
use hex;
use hmac::digest::KeyInit;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Block {
    pub index: u32,
    pub header: BlockHeader,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub signature: String,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u64,
        proposer_id: String,
        height: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        shared_key: String,
    ) -> Self {
        let header = BlockHeader {
            height,
            parent_hash: previous_hash,
            proposer_id: proposer_id.clone(),
            tx_count: transactions.len(),
        };
        let mut block = Self {
            index,
            header,
            timestamp,
            transactions,
            signature: String::new(),
            hash: String::new(),
        };

        block.hash = block.calculate_hash();

        let mut mac = <HmacSha256 as KeyInit>::new_from_slice(shared_key.as_bytes())
            .expect("HMAC new from slice failed");
        mac.update(block.hash.as_bytes());
        block.signature = hex::encode(mac.finalize().into_bytes());
        block
    }

    pub fn verify_signature(&self, shared_key: &str) -> bool {
        let mut mac = <HmacSha256 as KeyInit>::new_from_slice(shared_key.as_bytes())
            .expect("HMAC new from slice failed");

        mac.update(self.hash.as_bytes());

        let received_bytes = match hex::decode(&self.signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        mac.verify_slice(&received_bytes).is_ok()
    }

    pub fn calculate_hash(&self) -> String {
        let transactions_string = self
            .transactions
            .iter()
            .map(|t| format!("{}{}{}", t.from, t.to, t.amount))
            .collect::<Vec<String>>()
            .join("");

        let header_json = serde_json::to_string(&self.header).unwrap();

        let s = format!(
            "{}{}{}{}",
            self.index, self.timestamp, transactions_string, header_json
        );

        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let result_bytes = hasher.finalize();

        hex::encode(result_bytes)
    }
}
