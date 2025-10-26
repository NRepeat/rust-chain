use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    pub index: u32,
    timestamp: u64,
    data: Vec<u8>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
}

impl Block {
    pub fn new(prev_index: u32, prev_hash: String, data: Vec<u8>) -> Self {
        let mut block = Block {
            index: prev_index + 1,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            previous_hash: prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let s = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            String::from_utf8_lossy(&self.data),
            self.previous_hash,
            self.nonce
        );
        return digest(s.as_bytes());
    }
    pub fn mine_block(&mut self, difficulty: u32) {
        let target = "0".repeat(difficulty as usize);
        loop {
            let hash = self.calculate_hash();
            if hash.starts_with(&target) {
                println!("Блок ЗАМАЙНЕНО! Nonce: {}, Хеш: {}", self.nonce, hash);
                self.hash = hash;
                break;
            } else {
                self.nonce += 1;
            }
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        let index = 0;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let data = vec![];
        let previous_hash = String::from("0");
        let nonce = 0;
        let hash = digest(format!(
            "{}{}{}{}{}",
            index,
            timestamp,
            String::from_utf8_lossy(&data),
            previous_hash,
            nonce
        ));
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }
}
