use crate::api::dtos::{CreateTransactionDto, CreateUserDto};
use crate::blockchain::use_cases::add_block_to_chain::add_block_to_chain;
use crate::blockchain::use_cases::sync_chain_task::sync_chain_task;
use crate::domain::blockchain_repository::BlockchainRepository;
use crate::domain::mempool_repository::MempoolRepository;
use crate::domain::user_state_repository::UserStateRepository;
use crate::domain::vote::Vote;
use crate::domain::{app_state::AppState, block::Block, transaction::Transaction};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn get_all_blocks_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
) -> Json<Vec<Block>>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    let repo = app_state.blockchain_repo.lock().await;

    let blocks = repo.get_all_blocks().await;
    Json(blocks)
}

pub async fn get_all_transactions_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
) -> Json<Vec<Transaction>>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    let mempool_repo = app_state.mempool_repo.lock().await;
    let transactions = mempool_repo
        .get_all_transactions()
        .clone()
        .into_iter()
        .collect();
    Json(transactions)
}

pub async fn get_balance_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
    Path(address): Path<Uuid>,
) -> Json<Value>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    let user_state_repo = app_state.user_state_repo.lock().await;
    let balance = user_state_repo.get_balance(&address);
    Json(json!({ "balance": balance }))
}

pub async fn get_all_balances_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
) -> Json<HashMap<Uuid, f64>>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    let user_state_repo = app_state.user_state_repo.lock().await;
    let balances = user_state_repo.get_balances().clone();
    Json(balances)
}

pub async fn create_transaction_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
    Json(payload): Json<CreateTransactionDto>,
) -> impl IntoResponse
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    if payload.from == payload.to {
        return (
            StatusCode::BAD_REQUEST,
            "Sender and receiver addresses cannot be the same",
        )
            .into_response();
    }

    if payload.amount <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            "Transaction amount must be positive",
        )
            .into_response();
    }

    if payload.amount.is_nan() || payload.amount.is_infinite() {
        return (
            StatusCode::BAD_REQUEST,
            "Transaction amount cannot be NaN or infinite",
        )
            .into_response();
    }

    let user_state_repo = app_state.user_state_repo.lock().await;

    if !user_state_repo.get_balances().contains_key(&payload.from) {
        return (StatusCode::NOT_FOUND, "Sender not found").into_response();
    }

    if !user_state_repo.get_balances().contains_key(&payload.to) {
        return (StatusCode::NOT_FOUND, "Receiver not found").into_response();
    }

    let sender_balance = user_state_repo.get_balance(&payload.from);
    if sender_balance < payload.amount {
        return (StatusCode::BAD_REQUEST, "Insufficient balance").into_response();
    }

    let transaction = Transaction::new(payload.from, payload.to, payload.amount);
    let mut mempool_repo = app_state.mempool_repo.lock().await;

    if mempool_repo.check_exists_by_id(&transaction.id) {
        return (StatusCode::CONFLICT, "Transaction already exists").into_response();
    }
    mempool_repo.add_transaction(transaction.clone());
    (StatusCode::CREATED, Json(transaction)).into_response()
}
pub async fn create_user_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
    Json(payload): Json<CreateUserDto>,
) -> Json<Value>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    let mut user_state_repo = app_state.user_state_repo.lock().await;
    let new_user_id = Uuid::new_v4();
    user_state_repo.set_balance(new_user_id, payload.balance);

    let faucet_wallet_str =
        env::var("FAUCET_WALLET_ID").expect("FAUCET_WALLET_ID must be set in .env");
    let faucet_wallet_id =
        Uuid::parse_str(&faucet_wallet_str).expect("Failed to parse FAUCET_WALLET_ID");

    let new_user_id = Uuid::new_v4();
    let funding_tx = Transaction {
        id: Uuid::new_v4(),
        from: faucet_wallet_id,
        to: new_user_id,
        amount: payload.balance,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    {
        let mut mempool = app_state.mempool_repo.lock().await;
        mempool.add_transaction(funding_tx);
    }

    println!(
        "[API /user]: 💸 Funding transaction created for new user {}",
        new_user_id
    );

    Json(json!({ "id": new_user_id }))
}

pub async fn accept_block_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
    Json(received_block): Json<Block>,
) -> (StatusCode, String)
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    println!(
        "[API /block]: 📥 Отримано блок #{} від {}",
        received_block.header.height, received_block.header.proposer_id
    );

    let expected_hash = received_block.calculate_hash();
    if received_block.hash != expected_hash {
        println!("[API /block]: ❌ ВІДХИЛЕНО: Неправильний хеш!");
        return (StatusCode::BAD_REQUEST, "Invalid block hash".to_string());
    }

    let shared_key = app_state.shared_key.clone();
    if !received_block.verify_signature(&shared_key) {
        println!("[API /block]: ❌ ВІДХИЛЕНО: Неправильний підпис!");
        return (
            StatusCode::BAD_REQUEST,
            "Invalid block signature".to_string(),
        );
    }

    let mut blockchain = app_state.blockchain_repo.lock().await;

    let last_block = blockchain.get_last_block().await;
    {
        if received_block.header.parent_hash == last_block.hash
            && received_block.header.height == last_block.header.height + 1
        {
            {
                let mut user_state = app_state.user_state_repo.lock().await;
                for tx in &received_block.transactions {
                    if !user_state.apply_transaction(tx) {
                        println!(
                            "[API /block]: ❌ ВІДХИЛЕНО: Блок содержит невалидную транзакцию {}.",
                            tx.id
                        );

                        return (
                            StatusCode::BAD_REQUEST,
                            "Block contains invalid transactions".to_string(),
                        );
                    }
                }
            }

            println!(
                "[API /block]: ✅ Блок #{} пройшов усі перевірки.",
                received_block.header.height
            );

            let block_hash = received_block.hash.clone();
            blockchain.add_block(received_block).await;

            drop(blockchain);

            let (my_id, peer_addresses) = {
                let node = app_state.node.lock().await;
                (node.id.clone(), node.peers.clone())
            };

            let vote = Vote {
                block_hash: block_hash,
                voter_id: my_id,
                decision: "ACK".to_string(),
            };

            let http_client = app_state.http_client;
            for peer_addr in &peer_addresses {
                let target_url = format!("http://{}/vote", peer_addr);
                let _ = http_client.post(&target_url).json(&vote).send().await;
            }

            return (StatusCode::OK, "Block accepted, ACK sent".to_string());
        } else if received_block.header.height > last_block.header.height {
            println!(
                "[API /block]: 🍴 КОНФЛІКТ (FORK)! Наша висота {}, отримано {}.",
                last_block.header.height, received_block.header.height
            );

            tokio::spawn(sync_chain_task(app_state.clone()));

            return (
                StatusCode::CONFLICT,
                "Fork detected, starting sync".to_string(),
            );
        } else {
            println!("[API /block]: ❌ ВІДХИЛЕНО: Блок належить до коротшого ланцюга.");
            return (
                StatusCode::BAD_REQUEST,
                "Block is from a shorter chain".to_string(),
            );
        }
    }
}

pub async fn accept_vote_handler<B, M, U>(
    State(app_state): State<AppState<B, M, U>>,
    Json(vote): Json<Vote>,
) -> (StatusCode, String)
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    if vote.decision != "ACK" {
        return (StatusCode::OK, "Vote received (NACK)".to_string());
    }
    let total_validators = {
        let node = app_state.node.lock().await;
        node.validator_ids.len()
    };
    let quorum_needed = (total_validators / 2) + 1;
    let mut vote_counts = app_state.vote_counts.lock().await;
    let voters_for_this_block = vote_counts
        .entry(vote.block_hash.clone())
        .or_insert_with(Vec::new);
    if !voters_for_this_block.contains(&vote.voter_id) {
        voters_for_this_block.push(vote.voter_id.clone());
    }
    let current_vote_count = voters_for_this_block.len();
    let my_id = app_state.node.lock().await.id.clone();
    println!(
        "[API /vote]: 📥 (Я {}) Отримано голос від {}: {} за блок ...{}",
        my_id,
        vote.voter_id,
        vote.decision,
        &vote.block_hash[..5]
    );
    if current_vote_count >= quorum_needed {
        println!(
            "[API /vote]: КВОРУМ ЗІБРАНО! Лідер додає блок ...{}!",
            &vote.block_hash[..5]
        );

        let block_to_add = {
            let mut pending_blocks = app_state.pending_blocks.lock().await;
            pending_blocks.remove(&vote.block_hash)
        };

        if let Some(block) = block_to_add {
            add_block_to_chain(app_state.blockchain_repo.clone(), block.clone()).await;
            println!(
                "[API /vote]: ✅ Лідер успішно додав блок #{} до свого ланцюга.",
                block.clone().header.height
            );
            vote_counts.remove(&vote.block_hash);
        } else {
            println!(
                "[API /vote]: ⚠️ Блок ...{} вже був доданий.",
                &vote.block_hash[..5]
            );
        }
    }
    (StatusCode::OK, "Vote received".to_string())
}
