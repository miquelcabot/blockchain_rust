use blockchain_rust::block::Block;

fn main() {
    let block = Block::new_block("".to_string(), &[], 0);
    println!("{:?}", block);
}
