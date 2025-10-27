use axum::{Router, routing::get};

use crate::api::handlers::get_all_blocks_handler;
use crate::domain::mempool_repository::MempoolRepository;
use crate::domain::user_state_repository::UserStateRepository;
use crate::{
    domain::blockchain_repository::BlockchainRepository,
    infrastructure::{
        in_memory_blockchain_repository::InMemoryBlockchainRepository,
        in_memory_user_state_repository::InMemoryUserStateRepository,
        mempool_repository::InMemoryMempoolRepository,
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState<B, M, U>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    pub blockchain_repo: Arc<Mutex<B>>,
    pub mempool_repo: Arc<Mutex<M>>,
    pub user_state_repo: Arc<Mutex<U>>,
}

impl<B, M, U> Clone for AppState<B, M, U>
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            blockchain_repo: Arc::clone(&self.blockchain_repo),
            mempool_repo: Arc::clone(&self.mempool_repo),
            user_state_repo: Arc::clone(&self.user_state_repo),
        }
    }
}

pub async fn app(port: u16) {
    let blockchain_repo = InMemoryBlockchainRepository::new();
    let mempool_repo = InMemoryMempoolRepository::new();
    let user_state_repo = InMemoryUserStateRepository::new();

    let shared_blockchain_repo = Arc::new(Mutex::new(blockchain_repo));
    let shared_mempool_repo = Arc::new(Mutex::new(mempool_repo));
    let shared_user_state_repo = Arc::new(Mutex::new(user_state_repo));

    let app_state = AppState {
        blockchain_repo: shared_blockchain_repo,
        mempool_repo: shared_mempool_repo,
        user_state_repo: shared_user_state_repo,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/blocks", get(get_all_blocks_handler))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
