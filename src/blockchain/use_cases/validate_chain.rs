use crate::domain::blockchain_repository::BlockchainRepository;

pub fn validate_chain(blockchain_repository: &dyn BlockchainRepository) -> bool {
    let blocks = blockchain_repository.get_all_blocks();
    for (i, block) in blocks.iter().enumerate() {
        if i > 0 {
            let prev_block = &blocks[i - 1];
            if block.previous_hash != prev_block.hash {
                return false;
            }
        }
        if block.hash != block.calculate_hash() {
            return false;
        }
    }
    true
}
