use super::*;

#[derive(Debug)]

pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
}

impl Block {
    pub fn new(transaction: Vec<Transaction>) -> Self {
        let time = now();
        let empty_string = String::new();
        Block {
            timestamp: time,
            hash: empty_string.clone(),
            pre_hash: empty_string.clone(),
            transaction,
        }
    }

    pub fn set_pre_hash(&mut self, pre_hash: String) {
        self.hash = pre_hash;
    }
    pub fn set_hash(&mut self) {
        self.hash = calculate_hash(&self.pre_hash, &self.transaction, &self.timestamp)
    }
}
