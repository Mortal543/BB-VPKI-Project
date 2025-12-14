use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trusted Platform Module - Secure key storage and signing
pub struct TrustedPlatformModule {
    private_keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    #[allow(dead_code)]
    attestation_key: SigningKey,
}

impl TrustedPlatformModule {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        Self {
            private_keys: Arc::new(RwLock::new(HashMap::new())),
            attestation_key: SigningKey::generate(&mut csprng),
        }
    }

    pub async fn generate_key_pair(&self, key_id: &str) -> Vec<u8> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();

        self.private_keys
            .write()
            .await
            .insert(key_id.to_string(), signing_key.to_bytes().to_vec());

        public_key
    }

    pub async fn sign_with_tpm(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, String> {
        let keys = self.private_keys.read().await;
        if let Some(secret_bytes) = keys.get(key_id) {
            if secret_bytes.len() == 32 {
                if let Ok(secret_array) = <&[u8; 32]>::try_from(secret_bytes.as_slice()) {
                    let signing_key = SigningKey::from_bytes(secret_array);
                    return Ok(signing_key.sign(data).to_bytes().to_vec());
                }
            }
        }
        Err("Key not found in TPM".to_string())
    }

    pub async fn secure_erase(&self, key_id: &str) -> bool {
        self.private_keys.write().await.remove(key_id).is_some()
    }

    // pub async fn get_attestation_key(&self) -> Vec<u8> {
    //     self.attestation_key.verifying_key().to_bytes().to_vec()
    // }
}
