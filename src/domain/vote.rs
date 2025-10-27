use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub block_hash: String,
    pub voter_id: String,
    pub decision: String,
}
