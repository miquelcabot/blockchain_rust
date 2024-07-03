#[derive(Debug)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

#[derive(Debug)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

#[derive(Debug)]
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}
