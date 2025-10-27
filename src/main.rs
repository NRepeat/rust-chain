mod blockchain;
mod domain;
mod infrastructure;

use crate::domain::mempool::Mempool;
use blockchain::use_cases::add_new_block::add_new_block;
use blockchain::use_cases::create_genesis_block::create_genesis_block;
use blockchain::use_cases::process_mempool::process_mempool;
use blockchain::use_cases::validate_chain::validate_chain;
use domain::blockchain_repository::BlockchainRepository;
use domain::transaction::Transaction;
use domain::user_state_repository::UserStateRepository;
use infrastructure::in_memory_blockchain_repository::InMemoryBlockchainRepository;
use infrastructure::in_memory_user_state_repository::InMemoryUserStateRepository;
use infrastructure::mempool_repository::InMemoryMempoolRepository;
use uuid::Uuid;

fn main() {
    const DIFFICULTY: u32 = 2;

    // --- Repositories ---
    let mut blockchain_repo = InMemoryBlockchainRepository::new();
    let mempool_repo = InMemoryMempoolRepository::new();
    let mut user_state_repo = InMemoryUserStateRepository::new();

    // --- Initial Balances ---
    let user1 = Uuid::new_v4();
    let user2 = Uuid::new_v4();
    let user3 = Uuid::new_v4();
    user_state_repo.set_balance(user1, 100.0);
    user_state_repo.set_balance(user2, 50.0);

    println!("Initial balances: {:?}", user_state_repo.get_balances());

    // --- Mempool and Transactions ---
    let mut mempool = Mempool::new(mempool_repo);

    // Valid transaction
    let tx1 = Transaction::new(user1, user2, 25.0);
    mempool.add_transaction(tx1);

    // Invalid transaction (insufficient funds)
    let tx2 = Transaction::new(user2, user1, 60.0);
    mempool.add_transaction(tx2);

    // Valid transaction
    let tx3 = Transaction::new(user1, user3, 15.0);
    mempool.add_transaction(tx3);

    println!(
        "Mempool before processing: {:?}",
        mempool.get_all_transactions()
    );

    // --- Genesis Block ---
    create_genesis_block(&mut blockchain_repo, DIFFICULTY);

    // --- Process Mempool ---
    let processed_transactions = process_mempool(&mut mempool, &mut user_state_repo);
    println!("Processed transactions: {:?}", processed_transactions);
    println!(
        "Mempool after processing: {:?}",
        mempool.get_all_transactions()
    );
    println!(
        "Balances after processing: {:?}",
        user_state_repo.get_balances()
    );

    // --- Create New Block ---
    if !processed_transactions.is_empty() {
        add_new_block(&mut blockchain_repo, processed_transactions, DIFFICULTY);
    }

    // --- Display Blockchain ---
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
