use crate::blockchain::BlockchainTransaction;
use async_trait::async_trait;

#[async_trait]
pub trait LedgerGateway: Send + Sync {
    async fn connect(&self) -> Result<(), String>;
    async fn submit_transaction(&self, tx: &BlockchainTransaction) -> Result<(), String>;
}
