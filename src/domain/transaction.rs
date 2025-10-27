use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub from: Uuid,
    pub to: Uuid,
    pub amount: f64,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(from: Uuid, to: Uuid, amount: f64) -> Self {
        let id = Uuid::new_v4();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            id,
            from,
            to,
            amount,
            timestamp,
        }
    }
    pub fn is_valid(&self) -> bool {
        return self.from != self.to
            && !self.amount.is_nan()
            && !self.amount.is_infinite()
            && self.amount > 0.0;
    }
}
