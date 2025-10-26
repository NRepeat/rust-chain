mod entities;
use entities::block_chain::BlockChain;

fn main() {
    let mut my_chain = BlockChain::new();
    my_chain.mine();

    println!("\n--- Blockchain Details ---");
    for block in &my_chain.blocks {
        println!("Index: {}", block.index);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        println!("Data: {}", String::from_utf8_lossy(&block.data));
        let is_valid = BlockChain::validate_block(block, 2);
        println!("Is Block Valid? {}", is_valid);
        println!("--------------------------");
    }
}
