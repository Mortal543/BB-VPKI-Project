use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainTransaction {
    pub tx_id: String,
    pub tx_type: TransactionType,
    pub timestamp: DateTime<Utc>,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    CertificateIssuance,
    CertificateRevocation,
    CertificateRenewal,
    DeprecationArchive,
}

impl BlockchainTransaction {
    pub fn new(tx_id: String, tx_type: TransactionType, data: Vec<u8>) -> Self {
        Self {
            tx_id,
            tx_type,
            timestamp: Utc::now(),
            data,
            signature: vec![],
        }
    }
}
