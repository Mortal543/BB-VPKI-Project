use super::obu::OnBoardUnit;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BBVPKIClientSDK {
    obu: Arc<Mutex<OnBoardUnit>>,
}

impl BBVPKIClientSDK {
    pub async fn new(vehicle_id: String) -> Self {
        let obu = Arc::new(Mutex::new(OnBoardUnit::new(vehicle_id).await));
        Self { obu }
    }

    pub async fn initialize(&mut self) -> Result<(), String> {
        println!(
            "Client SDK initialized for vehicle: {}",
            self.obu.lock().await.vehicle_id
        );
        Ok(())
    }

    pub async fn sign_v2v_message(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        self.obu.lock().await.sign_message(message).await
    }

    #[allow(dead_code)]
    pub async fn get_vehicle_id(&self) -> String {
        self.obu.lock().await.vehicle_id.clone()
    }
}
