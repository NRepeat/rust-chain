pub struct Node {
    pub id: String,
    pub port: u16,
    pub peers: Vec<String>,
    pub validator_ids: Vec<String>,
}

impl Node {
    pub fn new(id: String, port: u16, peers: Vec<String>, validator_ids: Vec<String>) -> Self {
        Node {
            id,
            port,
            peers,
            validator_ids,
        }
    }
}
