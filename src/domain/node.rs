pub struct Node {
    pub id: String,
    pub peers: Vec<String>,
    pub validator_ids: Vec<String>,
}

impl Node {
    pub fn new(id: String, peers: Vec<String>, validator_ids: Vec<String>) -> Self {
        Node {
            id,
            peers,
            validator_ids,
        }
    }
}
