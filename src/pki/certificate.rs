use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub id: String,
    pub vehicle_id: String,
    pub public_key: Vec<u8>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub issuer_ca: String,
    pub status: CertificateStatus,
    pub certificate_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificateStatus {
    Active,
    Revoked,
    Expired,
    Deprecated,
}

impl Certificate {
    pub fn is_valid(&self) -> bool {
        self.status == CertificateStatus::Active && self.expires_at > Utc::now()
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}
