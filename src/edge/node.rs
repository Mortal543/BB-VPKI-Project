use crate::blockchain::Blockchain;
use crate::pki::CertificateStatus;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

pub struct EdgeNode {
    pub node_id: String,
    cache: Arc<RwLock<LruCache<String, (CertificateStatus, Instant)>>>,
    blockchain_ref: Arc<Blockchain>,
    cache_hits: Arc<AtomicU64>,
    cache_misses: Arc<AtomicU64>,
    neighboring_nodes: Arc<RwLock<Vec<String>>>,
}

impl EdgeNode {
    pub fn new(node_id: String, cache_size: usize, blockchain: Arc<Blockchain>) -> Self {
        Self {
            node_id,
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(cache_size).unwrap(),
            ))),
            blockchain_ref: blockchain,
            cache_hits: Arc::new(AtomicU64::new(0)),
            cache_misses: Arc::new(AtomicU64::new(0)),
            neighboring_nodes: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn authenticate_certificate(
        &self,
        cert_id: &str,
    ) -> Result<(CertificateStatus, u128), String> {
        let start = Instant::now();

        {
            let mut cache = self.cache.write().await;
            if let Some((status, _)) = cache.get(cert_id) {
                self.cache_hits.fetch_add(1, Ordering::Relaxed);
                let latency = start.elapsed().as_nanos();
                return Ok((status.clone(), latency));
            }
        }

        self.cache_misses.fetch_add(1, Ordering::Relaxed);

        let status = self.query_blockchain(cert_id).await?;

        self.cache
            .write()
            .await
            .put(cert_id.to_string(), (status.clone(), Instant::now()));

        let latency = start.elapsed().as_nanos();
        Ok((status, latency))
    }

    async fn query_blockchain(&self, cert_id: &str) -> Result<CertificateStatus, String> {
        tokio::time::sleep(Duration::from_micros(100)).await;

        let chain = self.blockchain_ref.chain.read().await;
        for block in chain.iter().rev() {
            for tx in &block.transactions {
                if tx.tx_id.contains(cert_id) {
                    return Ok(CertificateStatus::Active);
                }
            }
        }
        Err("Certificate not found".to_string())
    }

    pub async fn propagate_revocation(&self, cert_id: &str) {
        self.cache.write().await.put(
            cert_id.to_string(),
            (CertificateStatus::Revoked, Instant::now()),
        );
    }

    pub async fn get_cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;

        if total == 0 {
            return 0.0;
        }

        (hits as f64 / total as f64) * 100.0
    }

    pub async fn add_neighboring_node(&self, node_id: String) {
        self.neighboring_nodes.write().await.push(node_id);
    }
}
