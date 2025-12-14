use super::certificate::{Certificate, CertificateStatus};
use crate::crypto::HardwareSecurityModule;
use chrono::{DateTime, Duration, Utc};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CertificateAuthority {
    pub ca_id: String,
    hsm: Arc<HardwareSecurityModule>,
    issued_certificates: Arc<RwLock<HashMap<String, Certificate>>>,
    revocation_list: Arc<RwLock<Vec<String>>>,
}

impl CertificateAuthority {
    pub async fn new(ca_id: String, hsm: Arc<HardwareSecurityModule>) -> Self {
        hsm.generate_ca_keypair(&ca_id).await;

        Self {
            ca_id,
            hsm,
            issued_certificates: Arc::new(RwLock::new(HashMap::new())),
            revocation_list: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn issue_certificate(&self, vehicle_id: String, public_key: Vec<u8>) -> Certificate {
        let cert_id = format!("CERT-{}-{}", vehicle_id, Utc::now().timestamp_millis());
        let issued_at = Utc::now();
        let expires_at = issued_at + Duration::days(365);

        let mut hasher = Sha256::new();
        hasher.update(&cert_id);
        hasher.update(&vehicle_id);
        hasher.update(&public_key);
        let certificate_hash = format!("{:x}", hasher.finalize());

        let cert = Certificate {
            id: cert_id.clone(),
            vehicle_id,
            public_key,
            issued_at,
            expires_at,
            issuer_ca: self.ca_id.clone(),
            status: CertificateStatus::Active,
            certificate_hash,
        };

        let cert_data = serde_json::to_vec(&cert).unwrap();
        let _ = self.hsm.sign_certificate(&self.ca_id, &cert_data).await;

        self.issued_certificates
            .write()
            .await
            .insert(cert_id, cert.clone());
        cert
    }

    pub async fn revoke_certificate(&self, cert_id: &str) -> Result<DateTime<Utc>, String> {
        let revocation_time = Utc::now();

        let mut certs = self.issued_certificates.write().await;
        if let Some(cert) = certs.get_mut(cert_id) {
            cert.status = CertificateStatus::Revoked;
            drop(certs);

            self.revocation_list.write().await.push(cert_id.to_string());
            Ok(revocation_time)
        } else {
            Err("Certificate not found".to_string())
        }
    }

    pub async fn deprecate_expired_certificates(&self) -> Vec<String> {
        let mut deprecated = Vec::new();
        let mut certs = self.issued_certificates.write().await;

        for (cert_id, cert) in certs.iter_mut() {
            if cert.is_expired() && cert.status == CertificateStatus::Active {
                cert.status = CertificateStatus::Deprecated;
                deprecated.push(cert_id.clone());
            }
        }

        deprecated
    }

    pub async fn get_certificate(&self, cert_id: &str) -> Option<Certificate> {
        self.issued_certificates.read().await.get(cert_id).cloned()
    }

    pub async fn get_total_issued(&self) -> usize {
        self.issued_certificates.read().await.len()
    }
}
