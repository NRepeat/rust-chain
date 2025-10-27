use crate::domain::mempool::Mempool;
use crate::domain::mempool_repository::MempoolRepository;
use crate::domain::transaction::Transaction;
use crate::domain::user_state_repository::UserStateRepository;

pub fn process_mempool<M: MempoolRepository, S: UserStateRepository>(
    mempool: &mut Mempool<M>,
    user_state_repository: &mut S,
) -> Vec<Transaction> {
    let transactions = mempool.drain_transactions();
    let mut processed_transactions = Vec::new();

    for transaction in transactions {
        if user_state_repository.apply_transaction(&transaction) {
            processed_transactions.push(transaction);
        } else {
            println!(
                "Transaction discarded due to insufficient funds: {}",
                transaction.id
            );
        }
    }
    processed_transactions
}
