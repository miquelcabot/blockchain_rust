use std::sync::RwLock;

pub struct Nodes {
    inner: RwLock<Vec<Node>>,
}

pub struct Node {
    addr: String,
}
