use super::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
    difficulty: u32,
}

impl BlockChain {
    pub fn new() -> Self {
        let difficulty = 2;
        let mut genesis_block = Block::default();
        genesis_block.mine_block(difficulty);
        let block = vec![genesis_block];

        Self {
            blocks: block,
            difficulty: 2,
        }
    }

    pub fn mine(&mut self) {
        let transactions = vec![
            "Транзакція A -> B",
            "Транзакція B -> C",
            "Транзакція C -> A",
        ];

        for transaction_data in transactions {
            println!("\nГотуємо новий блок з даними: \"{}\"", transaction_data);

            self.add_block(transaction_data.to_string().into_bytes());
        }
    }
    pub fn add_block(&mut self, data: Vec<u8>) {
        let previous_block = self.blocks.last().unwrap();

        let new_index = previous_block.index + 1;

        let mut new_block = Block::new(new_index, previous_block.hash.clone(), data);

        println!(
            "Починаємо майнінг блоку {} (складність: {})...",
            new_index, self.difficulty
        );
        new_block.mine_block(self.difficulty);
        self.blocks.push(new_block);
        println!("Блок {} успішно замайнено та додано до ланцюга.", new_index);
    }

    pub fn validate_block(block: &Block, difficulty: u32) -> bool {
        let target = "0".repeat(difficulty as usize);
        block.hash.starts_with(&target)
    }
}
