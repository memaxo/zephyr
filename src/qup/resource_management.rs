use std::collections::{HashMap, BinaryHeap};
use std::sync::{Arc, Mutex};
use std::cmp::Reverse;
use etcd_client::{Client, GetOptions, PutOptions};
use std::time::Duration;
use tokio::time::interval;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub cpu: usize,
    pub gpu: usize,
    pub memory: usize,
}

pub struct ResourceManager {
    resources: Arc<Mutex<HashMap<usize, Resource>>>,
    scheduler: ResourceScheduler,
    etcd_client: Client,
    node_id: String,
}

impl ResourceManager {
    pub async fn new(etcd_endpoints: Vec<String>, node_id: String) -> Self {
        let etcd_client = Client::connect(etcd_endpoints, None).await.unwrap();
        ResourceManager {
            resources: Arc::new(Mutex::new(HashMap::new())),
            scheduler: ResourceScheduler::new(),
            etcd_client,
            node_id,
        }
    }

    pub async fn add_node(&self, node_id: usize, resource: Resource) {
        let mut resources = self.resources.lock().unwrap();
        resources.insert(node_id, resource.clone());
        self.etcd_client.put(node_id.to_string(), serde_json::to_string(&resource).unwrap(), None).await.unwrap();
    }

    pub async fn remove_node(&self, node_id: usize) {
        let mut resources = self.resources.lock().unwrap();
        resources.remove(&node_id);
        self.etcd_client.delete(node_id.to_string(), None).await.unwrap();
    }

    pub async fn allocate_resources(&self, required: Resource) -> Option<usize> {
        let resources = self.resources.lock().unwrap();
        self.scheduler.allocate(resources.clone(), required)
    }

    pub async fn update_resource(&self, node_id: usize, resource: Resource) {
        let mut resources = self.resources.lock().unwrap();
        if let Some(node_resource) = resources.get_mut(&node_id) {
            node_resource.cpu = resource.cpu;
            node_resource.gpu = resource.gpu;
            node_resource.memory = resource.memory;
            self.etcd_client.put(node_id.to_string(), serde_json::to_string(&resource).unwrap(), None).await.unwrap();
        }
    }

    pub async fn get_resources(&self) -> HashMap<usize, Resource> {
        let resources = self.resources.lock().unwrap();
        resources.clone()
    }

    pub async fn start_heartbeat(&self) {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            self.etcd_client.put(format!("heartbeat/{}", self.node_id), "alive", Some(PutOptions::new().with_lease(60))).await.unwrap();
        }
    }

    pub async fn monitor_heartbeats(&self) {
        let mut interval = interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let response = self.etcd_client.get("heartbeat/", Some(GetOptions::new().with_prefix())).await.unwrap();
            for kv in response.kvs() {
                let node_id = kv.key_str().unwrap().replace("heartbeat/", "");
                if kv.value_str().unwrap() != "alive" {
                    println!("Node {} is down", node_id);
                    // Trigger failover logic here
                }
            }
        }
    }
}

pub struct ResourceScheduler;

impl ResourceScheduler {
    pub fn new() -> Self {
        ResourceScheduler
    }

    pub fn allocate(&self, resources: HashMap<usize, Resource>, required: Resource) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        for (node_id, resource) in resources.iter() {
            if resource.cpu >= required.cpu && resource.gpu >= required.gpu && resource.memory >= required.memory {
                heap.push(Reverse((resource.cpu + resource.gpu + resource.memory, *node_id)));
            }
        }

        heap.pop().map(|Reverse((_, node_id))| node_id)
    }
}
