use crate::block::Block;
use num_bigint::BigInt;

#[derive(Debug)]
pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}
