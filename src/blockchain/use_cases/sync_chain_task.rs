use crate::domain::{
    app_state::AppState, block::Block, blockchain_repository::BlockchainRepository,
    mempool_repository::MempoolRepository, user_state_repository::UserStateRepository,
};

// Фонове завдання для синхронізації
pub async fn sync_chain_task<B, M, U>(app_state: AppState<B, M, U>)
where
    B: BlockchainRepository + Send + Sync + 'static,
    M: MempoolRepository + Send + Sync + 'static,
    U: UserStateRepository + Send + Sync + 'static,
{
    println!("[Sync]: 🔄 Починаємо синхронізацію ланцюга...");

    let (peers, http_client) = {
        (
            app_state.node.lock().await.peers.clone(),
            app_state.http_client.clone(),
        )
    };

    let mut longest_chain: Vec<Block> = Vec::new();

    for peer_addr in &peers {
        let target_url = format!("http://{}/blocks", peer_addr);

        match http_client.get(&target_url).send().await {
            Ok(response) => {
                if let Ok(peer_chain) = response.json::<Vec<Block>>().await {
                    if peer_chain.len() > longest_chain.len() {
                        longest_chain = peer_chain;
                    }
                }
            }
            Err(e) => println!("[Sync]: ⚠️ Не вдалося підключитися до {}: {}", peer_addr, e),
        }
    }

    if longest_chain.is_empty() {
        println!("[Sync]: ❌ Не вдалося знайти довший ланцюг у пірів.");
        return;
    }

    println!(
        "[Sync]: 💾 Знайдено довший ланцюг (висота {}). Замінюємо локальний...",
        longest_chain.len() - 1
    );

    let mut blockchain = app_state.blockchain_repo.lock().await;
    let mut user_state = app_state.user_state_repo.lock().await;

    blockchain.replace_chain(longest_chain.clone()).await;

    user_state.rebuild_from_blocks(&longest_chain).await;

    println!("[Sync]: ✅ Синхронізацію завершено!");
}
