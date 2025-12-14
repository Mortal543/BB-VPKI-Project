use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Hardware Security Module for CA operations
pub struct HardwareSecurityModule {
    ca_keys: Arc<RwLock<HashMap<String, SigningKey>>>,
    operations_log: Arc<RwLock<Vec<String>>>,
}

impl HardwareSecurityModule {
    pub fn new() -> Self {
        Self {
            ca_keys: Arc::new(RwLock::new(HashMap::new())),
            operations_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn generate_ca_keypair(&self, ca_id: &str) -> Vec<u8> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();

        self.ca_keys
            .write()
            .await
            .insert(ca_id.to_string(), signing_key);
        self.log_operation(&format!("CA keypair generated: {}", ca_id))
            .await;

        public_key
    }

    pub async fn sign_certificate(&self, ca_id: &str, cert_data: &[u8]) -> Result<Vec<u8>, String> {
        let keys = self.ca_keys.read().await;
        if let Some(keypair) = keys.get(ca_id) {
            let signature = keypair.sign(cert_data).to_bytes().to_vec();
            drop(keys);
            self.log_operation(&format!("Certificate signed by CA: {}", ca_id))
                .await;
            return Ok(signature);
        }
        Err("CA key not found in HSM".to_string())
    }

    async fn log_operation(&self, operation: &str) {
        self.operations_log
            .write()
            .await
            .push(operation.to_string());
    }

    pub async fn get_operation_count(&self) -> usize {
        self.operations_log.read().await.len()
    }
}
