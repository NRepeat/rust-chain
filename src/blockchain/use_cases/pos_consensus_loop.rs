use crate::{
    blockchain::use_cases::add_new_block::add_new_block,
    domain::{
        app_state::AppState, blockchain_repository::BlockchainRepository,
        mempool_repository::MempoolRepository, transaction::Transaction,
        user_state_repository::UserStateRepository,
    },
};
use std::{collections::VecDeque, time::Duration};

pub async fn pos_consensus_loop<B, M, U>(app_state: AppState<B, M, U>)
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    let slot_duration = Duration::from_secs(5);
    let mut current_slot: u64 = 0;

    let mut validator_list = vec!["v1", "v2", "v3"];
    validator_list.sort();

    let total_validators = validator_list.len();

    let my_id = {
        let node = app_state.node.lock().await;
        node.id.clone()
    };

    println!("[PoS ]: Id: {}, Validators: {:?}", my_id, validator_list);

    loop {
        tokio::time::sleep(slot_duration).await;
        current_slot += 1;

        let leader_index = (current_slot - 1) as usize % total_validators;
        let leader_id = validator_list[leader_index];

        println!("[Slot {}]: Leader - {}", current_slot, leader_id);

        if my_id == leader_id {
            println!(
                "[Slot {}]: ✅ I'm the LEADER. Forming a block...",
                current_slot
            );
            let transactions_deque: VecDeque<Transaction> = {
                let mut mempool = app_state.mempool_repo.lock().await;
                mempool.drain_transactions()
            };
            let transactions_vec: Vec<Transaction> = transactions_deque.into_iter().collect();
            let valid_transactions: Vec<Transaction> = transactions_vec
                .into_iter()
                .filter(|t| t.is_valid())
                .collect();
            if valid_transactions.is_empty() {
                println!(
                    "[Slot {}]: 🤷 Mempool is empty. Skipping slot.",
                    current_slot
                );
                continue;
            }
            println!(
                "[Slot {}]: 📦 Packed {} valid transactions.",
                current_slot,
                valid_transactions.len()
            );
            let new_block = add_new_block(
                app_state.blockchain_repo.clone(),
                valid_transactions,
                my_id.clone(),
                app_state.shared_key.clone(),
            )
            .await;
        } else {
            println!(
                "[Slot {}]: ⏳ I'm a VALIDATOR. Waiting for block from {}.",
                current_slot, leader_id
            );
        }
    }
}
