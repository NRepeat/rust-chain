use domain::transaction::Transaction;
use std::collections::VecDeque;
//двусторонняя очередь
#[derive(Debug, Clone)]
pub struct Mempool {
    pub transactions: VecDeque<Transaction>,
}
