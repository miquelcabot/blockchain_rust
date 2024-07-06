use serde::{Deserialize, Serialize};

pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pkcs8: Vec<u8>,
    public_key: Vec<u8>,
}
