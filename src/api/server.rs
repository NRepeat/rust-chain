use crate::api::args::Args;
use crate::api::handlers::get_all_blocks_handler;
use crate::domain::app_state::AppState;
use crate::domain::node::Node;
use crate::infrastructure::{
    in_memory_blockchain_repository::InMemoryBlockchainRepository,
    in_memory_user_state_repository::InMemoryUserStateRepository,
    mempool_repository::InMemoryMempoolRepository,
};
use axum::{Router, routing::get};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn app() {
    let args = Args::parse();
    println!("  -> Слушаю порт: {}", args.port);
    println!("  -> Пиры: {:?}", args.peers);
    let node = Node::new(uuid::Uuid::new_v4().to_string(), args.port, args.peers);

    let blockchain_repo = InMemoryBlockchainRepository::new();
    let mempool_repo = InMemoryMempoolRepository::new();
    let user_state_repo = InMemoryUserStateRepository::new();

    let shared_blockchain_repo = Arc::new(Mutex::new(blockchain_repo));
    let shared_mempool_repo = Arc::new(Mutex::new(mempool_repo));
    let shared_user_state_repo = Arc::new(Mutex::new(user_state_repo));
    let shared_node = Arc::new(Mutex::new(node));

    let app_state = AppState {
        blockchain_repo: shared_blockchain_repo,
        mempool_repo: shared_mempool_repo,
        user_state_repo: shared_user_state_repo,
        node: shared_node,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/blocks", get(get_all_blocks_handler))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
