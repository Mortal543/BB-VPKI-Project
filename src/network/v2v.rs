use crate::edge::EdgeNode;
use crate::vehicle::OnBoardUnit;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::{Mutex, RwLock};

pub struct V2VNetwork {
    nodes: Arc<RwLock<HashMap<String, Arc<EdgeNode>>>>,
    vehicles: Arc<RwLock<HashMap<String, Arc<Mutex<OnBoardUnit>>>>>,
    message_counter: Arc<AtomicUsize>,
}

impl V2VNetwork {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            vehicles: Arc::new(RwLock::new(HashMap::new())),
            message_counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub async fn register_edge_node(&self, node: Arc<EdgeNode>) {
        self.nodes.write().await.insert(node.node_id.clone(), node);
    }

    pub async fn register_vehicle(&self, vehicle: Arc<Mutex<OnBoardUnit>>) {
        let id = vehicle.lock().await.vehicle_id.clone();
        self.vehicles.write().await.insert(id, vehicle);
    }

    pub async fn broadcast_message(&self, sender_id: &str, _message: Vec<u8>) -> usize {
        self.message_counter.fetch_add(1, Ordering::Relaxed);

        let vehicles = self.vehicles.read().await;
        let mut delivered = 0;

        for (vehicle_id, _) in vehicles.iter() {
            if vehicle_id != sender_id {
                delivered += 1;
            }
        }

        delivered
    }

    pub fn get_message_count(&self) -> usize {
        self.message_counter.load(Ordering::Relaxed)
    }
}
