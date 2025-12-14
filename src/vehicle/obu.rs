use crate::crypto::TrustedPlatformModule;
use crate::pki::Certificate;
use ed25519_dalek::Verifier;
use ed25519_dalek::{Signature, VerifyingKey};
use std::sync::Arc;

pub struct OnBoardUnit {
    pub vehicle_id: String,
    tpm: Arc<TrustedPlatformModule>,
    key_id: String,
    certificate: Option<Certificate>,
    pub public_key: Vec<u8>,
}

impl OnBoardUnit {
    pub async fn new(vehicle_id: String) -> Self {
        let tpm = Arc::new(TrustedPlatformModule::new());
        let key_id = format!("TPM-KEY-{}", vehicle_id);
        let public_key = tpm.generate_key_pair(&key_id).await;

        Self {
            vehicle_id,
            tpm,
            key_id,
            certificate: None,
            public_key,
        }
    }

    pub async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        self.tpm.sign_with_tpm(&self.key_id, message).await
    }

    pub fn verify_message(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        if public_key.len() == 32 {
            if let Ok(pk) = VerifyingKey::from_bytes(<&[u8; 32]>::try_from(public_key).unwrap()) {
                if let Ok(sig_array) = <&[u8; 64]>::try_from(signature) {
                    let sig = Signature::from_bytes(sig_array);
                    return pk.verify(message, &sig).is_ok();
                }
            }
        }
        false
    }

    pub fn set_certificate(&mut self, cert: Certificate) {
        self.certificate = Some(cert);
    }

    pub async fn secure_erase_keys(&self) {
        self.tpm.secure_erase(&self.key_id).await;
    }

    pub fn get_certificate(&self) -> Option<&Certificate> {
        self.certificate.as_ref()
    }
}
