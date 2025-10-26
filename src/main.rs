mod blockchain;
mod domain;
mod infrastructure;

use blockchain::services::BlockchainRepository;
use blockchain::use_cases::add_new_block::add_new_block;
use blockchain::use_cases::create_genesis_block::create_genesis_block;
use blockchain::use_cases::validate_chain::validate_chain;
use domain::transaction::Transaction;
use infrastructure::in_memory_blockchain_repository::InMemoryBlockchainRepository;

fn main() {
    const DIFFICULTY: u32 = 2;
    let mut blockchain_repo = InMemoryBlockchainRepository::new();

    create_genesis_block(&mut blockchain_repo, DIFFICULTY);

    let transactions = vec![
        vec![
            Transaction {
                from: "A".to_string(),
                to: "B".to_string(),
                amount: 10,
            },
            Transaction {
                from: "B".to_string(),
                to: "C".to_string(),
                amount: 5,
            },
        ],
        vec![
            Transaction {
                from: "A".to_string(),
                to: "B".to_string(),
                amount: 10,
            },
            Transaction {
                from: "B".to_string(),
                to: "C".to_string(),
                amount: 5,
            },
        ],
    ];
    for transaction in transactions {
        add_new_block(&mut blockchain_repo, transaction, DIFFICULTY);
    }

    let blocks = blockchain_repo.get_all_blocks();

    for block in blocks {
        println!("Index: {}", block.index);
        println!("Timestamp: {}", block.timestamp);
        println!("Transactions: {:?}", block.transactions);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        println!("------------------------");
    }

    println!("Is chain valid? {}", validate_chain(&blockchain_repo));
}
