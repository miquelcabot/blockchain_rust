#[derive(Debug)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<u64>,
    nonce: u64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[u64], height: usize) -> Block {
        let mut block = Block {
            timestamp: 0,
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        // let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = (0, "".to_string()); // pow.run();
        block.nonce = nonce;
        block.hash = hash;
        block
    }
}
