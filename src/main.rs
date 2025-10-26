mod entities;
use entities::block_chain::BlockChain;

fn main() {
    let mut my_chain = BlockChain::new();
    my_chain.mine();

    println!("Мой блокчейн: {} блоков", my_chain.blocks.len());
    println!("Hello, world!");
}
