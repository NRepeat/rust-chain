mod blockchain;
mod domain;
mod infrastructure;

use blockchain::use_cases::add_new_block::add_new_block;
use blockchain::use_cases::create_genesis_block::create_genesis_block;
use blockchain::use_cases::validate_chain::validate_chain;
use domain::blockchain_repository::BlockchainRepository;
use domain::transaction::Transaction;
use infrastructure::in_memory_blockchain_repository::InMemoryBlockchainRepository;
use infrastructure::mempool_repository::InMemoryMempoolRepository;
use uuid::Uuid;

use crate::domain::mempool::Mempool;

fn main() {
    const DIFFICULTY: u32 = 2;
    let mut blockchain_repo = InMemoryBlockchainRepository::new();
    let mempool_repo = InMemoryMempoolRepository::new();
    let mut mempool = Mempool::new(mempool_repo);

    let sender = Uuid::new_v4();
    let receiver = Uuid::new_v4();
    let transaction = Transaction::new(sender, receiver, 10.0);
    mempool.add_transaction(transaction);

    mempool.get_all_transactions();
    create_genesis_block(&mut blockchain_repo, DIFFICULTY);

    let transactions = vec![];
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
