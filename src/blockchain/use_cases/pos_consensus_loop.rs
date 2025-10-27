use crate::{
    blockchain::use_cases::add_new_block::add_new_block,
    domain::{
        app_state::AppState, block::Block, blockchain_repository::BlockchainRepository,
        mempool_repository::MempoolRepository, transaction::Transaction,
        user_state_repository::UserStateRepository,
    },
};
use reqwest::Client;
use std::{collections::VecDeque, time::Duration};

pub async fn pos_consensus_loop<B, M, U>(app_state: AppState<B, M, U>)
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    let slot_duration = Duration::from_secs(5);
    let mut current_slot: u64 = 0;
    let http_client = Client::new();

    // vec!["v1", "v2", "v3"],
    let (my_id, peer_addresses, shared_key, mut validator_list) = {
        let node = app_state.node.lock().await;
        (
            node.id.clone(),
            node.peers.clone(),
            app_state.shared_key.clone(),
            node.validator_ids.clone(),
        )
    };
    println!("[PoS ]: Id: {}, Validators: {:?}", my_id, validator_list);
    validator_list.sort();

    let total_validators = validator_list.len();

    println!("[PoS ]: Id: {}, Validators: {:?}", my_id, validator_list);

    loop {
        tokio::time::sleep(slot_duration).await;
        current_slot += 1;

        let leader_index = (current_slot - 1) as usize % total_validators;
        let leader_id: String = validator_list[leader_index].clone();
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
            let mut user_state = app_state.user_state_repo.lock().await;
            println!("Genesis block created  {:?}", transactions_vec);
            let mut valid_transactions: Vec<Transaction> = Vec::new();

            for tx in transactions_vec {
                if user_state.apply_transaction(&tx) {
                    valid_transactions.push(tx);
                } else {
                    println!(
                        "[Slot {}]: Transaction {} rejected (insufficient funds).",
                        current_slot, tx.id
                    );
                }
            }

            if valid_transactions.is_empty() {
                println!("[Slot {}]: Mempool is empty. Skipping slot.", current_slot);
                continue;
            }
            println!(
                "[Slot {}]: 📦 Packed {} valid transactions.",
                current_slot,
                valid_transactions.len()
            );
            let new_block: Block = add_new_block(
                app_state.blockchain_repo.clone(),
                valid_transactions,
                &my_id,
                &shared_key,
            )
            .await;
            println!(
                "[Slot {}]:  Broadcasting block #{} to peers...",
                current_slot, new_block.header.height
            );
            for peer_addr in &peer_addresses {
                let target_url = format!("http://{}/block", peer_addr);
                println!("[Slot {}]: -> sending to {}", current_slot, target_url);
                let _ = http_client.post(&target_url).json(&new_block).send().await;
            }
        } else {
            println!(
                "[Slot {}]:  I'm a VALIDATOR. Waiting for block from {}.",
                current_slot, leader_id
            );
        }
    }
}
