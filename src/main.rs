mod block;
mod poh;

use block::Block;

fn main() {
    let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
    println!("{:?}", genesis_block);
    
    if genesis_block.verify() {
        println!("Block is valid!");
    } else {
        println!("Block is invalid!");
    }
}