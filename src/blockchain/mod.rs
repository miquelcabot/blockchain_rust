use sled::Db;
use std::sync::{Arc, RwLock};

pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,
}
