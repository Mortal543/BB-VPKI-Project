use crate::blockchain::BlockchainTransaction;
use crate::network::gateway::LedgerGateway;
use async_trait::async_trait;
use hex;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

pub struct HyperledgerFabricGateway {
    channel_name: String,
    chaincode_name: String,
    connected: Arc<Mutex<bool>>,
}

impl HyperledgerFabricGateway {
    pub fn new(channel_name: String, chaincode_name: String) -> Self {
        Self {
            channel_name,
            chaincode_name,
            connected: Arc::new(Mutex::new(false)),
        }
    }

    async fn connect_internal(&self) -> Result<(), String> {
        println!("Connecting to Hyperledger Fabric network...");
        println!("  → Channel: {}", self.channel_name);
        println!("  → Chaincode: {}", self.chaincode_name);

        sleep(Duration::from_millis(100)).await;
        let mut guard = self.connected.lock().await;
        *guard = true;

        println!("  ✓ Connected to HLF network");
        Ok(())
    }

    async fn submit_transaction_internal(
        &self,
        _function: &str,
        _args: Vec<String>,
    ) -> Result<Vec<u8>, String> {
        let guard = self.connected.lock().await;
        if !*guard {
            return Err("Not connected to HLF network".to_string());
        }

        // silent submit (no println for each tx to reduce noise during benchmarks)
        sleep(Duration::from_millis(50)).await;

        Ok(vec![])
    }
}

#[async_trait]
impl LedgerGateway for HyperledgerFabricGateway {
    async fn connect(&self) -> Result<(), String> {
        self.connect_internal().await
    }

    async fn submit_transaction(&self, tx: &BlockchainTransaction) -> Result<(), String> {
        // simulate submission: function name and args derived from tx
        let function = match tx.tx_type {
            crate::blockchain::TransactionType::CertificateIssuance => "issueCertificate",
            crate::blockchain::TransactionType::CertificateRevocation => "revokeCertificate",
            crate::blockchain::TransactionType::CertificateRenewal => "renewCertificate",
            crate::blockchain::TransactionType::DeprecationArchive => "archiveCertificate",
        };

        let args = vec![tx.tx_id.clone(), hex::encode(&tx.data)];
        self.submit_transaction_internal(function, args)
            .await
            .map(|_| ())
    }
}
