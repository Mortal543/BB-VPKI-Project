use super::block::Block;
use super::transaction::BlockchainTransaction;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Blockchain {
    pub chain: Arc<RwLock<Vec<Block>>>,
    pending_transactions: Arc<RwLock<Vec<BlockchainTransaction>>>,
    difficulty: u32,
    pruned_blocks: Arc<RwLock<HashMap<u64, String>>>,
    archived_certs: Arc<RwLock<HashMap<String, String>>>,
    consensus_latencies_ms: Arc<RwLock<Vec<u128>>>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let genesis = Block::genesis();

        Self {
            chain: Arc::new(RwLock::new(vec![genesis])),
            pending_transactions: Arc::new(RwLock::new(vec![])),
            difficulty,
            pruned_blocks: Arc::new(RwLock::new(HashMap::new())),
            archived_certs: Arc::new(RwLock::new(HashMap::new())),
            consensus_latencies_ms: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_transaction(&self, tx: BlockchainTransaction) {
        self.pending_transactions.write().await.push(tx);
    }

    pub async fn mine_pending_transactions(&self) {
        let pending = {
            let mut txs = self.pending_transactions.write().await;
            if txs.is_empty() {
                return;
            }
            let pending = txs.clone();
            txs.clear();
            pending
        };

        let chain = self.chain.read().await;
        let previous_block = chain.last().unwrap();
        let index = previous_block.index + 1;
        let previous_hash = previous_block.hash.clone();
        drop(chain);

        let mut block = Block::new(index, pending, previous_hash);

        loop {
            let hash = self.calculate_hash(&block);
            if self.is_valid_hash(&hash) {
                block.hash = hash;
                break;
            }
            block.nonce += 1;
        }

        // calculate consensus latencies: difference between block timestamp and each tx timestamp
        let mut latencies = Vec::new();
        for tx in &block.transactions {
            let diff = block
                .timestamp
                .signed_duration_since(tx.timestamp)
                .num_milliseconds();
            if diff >= 0 {
                latencies.push(diff as u128);
            }
        }

        if !latencies.is_empty() {
            let mut stored = self.consensus_latencies_ms.write().await;
            stored.extend(latencies);
            // keep vector bounded to last 1000 entries to avoid unbounded growth
            if stored.len() > 1000 {
                let start = stored.len() - 1000;
                *stored = stored[start..].to_vec();
            }
        }

        self.chain.write().await.push(block);
    }

    pub async fn get_average_consensus_latency_ms(&self) -> f64 {
        let stored = self.consensus_latencies_ms.read().await;
        if stored.is_empty() {
            return 0.0;
        }
        let sum: u128 = stored.iter().sum();
        (sum as f64) / (stored.len() as f64)
    }

    pub async fn get_consensus_percentiles_ms(&self) -> (f64, f64, f64) {
        let stored = self.consensus_latencies_ms.read().await;
        if stored.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        // work on a sorted copy
        let mut vals: Vec<u128> = stored.clone();
        vals.sort();
        let n = vals.len();
        let p = |quant: f64| -> usize {
            let idx = (quant * n as f64).ceil() as isize - 1;
            if idx < 0 {
                0usize
            } else if (idx as usize) >= n {
                n - 1
            } else {
                idx as usize
            }
        };

        let p50 = vals[p(0.50)] as f64;
        let p95 = vals[p(0.95)] as f64;
        let p99 = vals[p(0.99)] as f64;

        (p50, p95, p99)
    }

    fn calculate_hash(&self, block: &Block) -> String {
        let data = format!(
            "{}{}{}{}{}",
            block.index,
            block.timestamp,
            serde_json::to_string(&block.transactions).unwrap(),
            block.previous_hash,
            block.nonce
        );
        format!("{:x}", Sha256::digest(data.as_bytes()))
    }

    fn is_valid_hash(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty as usize))
    }

    pub async fn prune_old_blocks(&self, keep_last_n: usize) -> usize {
        let mut chain = self.chain.write().await;
        let chain_len = chain.len();

        if chain_len <= keep_last_n {
            return 0;
        }

        let to_prune = chain_len - keep_last_n;
        let mut pruned = self.pruned_blocks.write().await;

        for i in 1..to_prune {
            if let Some(block) = chain.get(i) {
                pruned.insert(block.index, block.hash.clone());
            }
        }

        chain.drain(1..to_prune);
        to_prune - 1
    }

    pub async fn archive_deprecated_certificate(&self, cert_id: String, cert_hash: String) {
        self.archived_certs.write().await.insert(cert_id, cert_hash);
    }

    pub async fn get_blockchain_size(&self) -> usize {
        let chain = self.chain.read().await;
        bincode::serialize(&*chain).unwrap_or_default().len()
    }

    pub async fn get_transaction_throughput(&self, duration_secs: u64) -> f64 {
        let chain = self.chain.read().await;
        let total_txs: usize = chain.iter().map(|b| b.transactions.len()).sum();
        if duration_secs == 0 {
            return 0.0;
        }
        total_txs as f64 / duration_secs as f64
    }

    pub async fn get_chain_length(&self) -> usize {
        self.chain.read().await.len()
    }
}
