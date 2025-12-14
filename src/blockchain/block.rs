use super::transaction::BlockchainTransaction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<BlockchainTransaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<BlockchainTransaction>,
        previous_hash: String,
    ) -> Self {
        Self {
            index,
            timestamp: Utc::now(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn genesis() -> Self {
        Self {
            index: 0,
            timestamp: Utc::now(),
            transactions: vec![],
            previous_hash: "0".to_string(),
            hash: "genesis_hash".to_string(),
            nonce: 0,
        }
    }
}
