use crate::api::dtos::{CreateTransactionDto, CreateUserDto};
use crate::domain::blockchain_repository::BlockchainRepository;
use crate::domain::mempool_repository::MempoolRepository;
use crate::domain::user_state_repository::UserStateRepository;
use crate::domain::{app_state::AppState, block::Block, transaction::Transaction};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{Value, json};
use std::collections::HashMap;
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
    if received_block.header.parent_hash != last_block.hash {
        println!("[API /block]: ❌ ВІДХИЛЕНО: Неправильний parent_hash (Fork?)");
        return (StatusCode::BAD_REQUEST, "Invalid parent hash".to_string());
    }
    println!(
        "[API /block]: ✅ Блок #{} пройшов перевірку.",
        received_block.header.height
    );
    blockchain.add_block(received_block).await;
    (StatusCode::OK, "Block accepted and added".to_string())
}
