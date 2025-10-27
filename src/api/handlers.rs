use crate::domain::block::Block;
use crate::domain::blockchain_repository::BlockchainRepository;
use crate::domain::user_state_repository::UserStateRepository;
use crate::{api::server::AppState, domain::mempool_repository::MempoolRepository};
use axum::{Json, extract::State};

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
