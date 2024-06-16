use std::collections::{HashMap, BinaryHeap, VecDeque};
use crate::utils::latency::{ping_nodes, Latency};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub load: f64,
    pub latency: f64,
    pub reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLatency {
    pub latencies: HashMap<(usize, usize), f64>,
}

use tokio::sync::RwLock;

pub struct ResourceManager {
    pub node_metrics: Arc<RwLock<HashMap<usize, NodeMetrics>>>,
    resources: Arc<RwLock<HashMap<usize, Resource>>>,
    scheduler: ResourceScheduler,
    etcd_client: Client,
    node_id: String,
    network_latency: Arc<RwLock<NetworkLatency>>,
    is_primary: Arc<RwLock<bool>>,
}

impl ResourceManager {
    pub async fn new(etcd_endpoints: Vec<String>, node_id: String) -> Self {
        let etcd_client = Client::connect(etcd_endpoints, None).await.unwrap();
        let node_metrics = Arc::new(RwLock::new(HashMap::new()));
        let network_latency = Arc::new(RwLock::new(NetworkLatency {
            latencies: HashMap::new(),
        }));
        let is_primary = Arc::new(RwLock::new(false));

        ResourceManager {
            network_latency,
            node_metrics,
            resources: Arc::new(RwLock::new(HashMap::new())),
            scheduler: ResourceScheduler::new(),
            etcd_client,
            node_id,
            is_primary,
        }
    }

    pub async fn start(&self) {
        self.start_heartbeat().await;
        self.monitor_primary().await;
    }

    async fn monitor_primary(&self) {
        let mut interval = interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let primary_key = "resource_manager_primary";
            let primary_node = self.etcd_client.get(primary_key, None).await.unwrap();

            if primary_node.count() == 0 {
                // No primary exists, initiate election
                self.initiate_election().await;
            } else if primary_node.kvs().next().unwrap().value_str().unwrap() != self.node_id {
                // Another node is primary, set is_primary to false
                let mut is_primary = self.is_primary.write().await;
                *is_primary = false;
            }
        }
    }

    async fn initiate_election(&self) {
        let election_key = "resource_manager_election";
        let lease = self.etcd_client.lease_grant(60, None).await.unwrap();
        let mut txn = Txn::new();
        txn.compare(CompareOp::create(), election_key, CompareTarget::version(0));
        txn.success().put(election_key, &self.node_id, Some(PutOptions::new().with_lease(lease.id())));
        let resp = self.etcd_client.txn(txn).await.unwrap();

        if resp.succeeded() {
            // Election won, set is_primary to true
            let mut is_primary = self.is_primary.write().await;
            *is_primary = true;
            self.etcd_client.put("resource_manager_primary", &self.node_id, None).await.unwrap();
        }
    }

    pub async fn is_primary(&self) -> bool {
        *self.is_primary.read().await
    }

    pub async fn allocate_resources(&self, required: Resource, task_priority: f64, current_node: usize) -> Option<usize> {
        if !self.is_primary().await {
            return None;
        }

        let resources = self.resources.read().await;
        let node_metrics = self.node_metrics.read().await;
        let network_latency = self.network_latency.read().await;
        self.scheduler.allocate(resources.clone(), node_metrics.clone(), required, task_priority, current_node, &network_latency)
    }

    pub async fn measure_network_latency(&self, nodes: Vec<NodeId>) -> HashMap<NodeId, Duration> {
        let latencies = ping_nodes(nodes);
        latencies.into_iter().map(|latency| {
            (NodeId::from(latency.node_id), latency.latency)
        }).collect()
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

    pub async fn allocate_resources(&self, required: Resource, task_priority: f64, current_node: usize) -> Option<usize> {
        let resources = self.resources.lock().unwrap();
        let node_metrics = self.node_metrics.lock().unwrap();
        let network_latency = self.network_latency.lock().unwrap();
        self.scheduler.allocate(resources.clone(), node_metrics.clone(), required, task_priority, current_node, &network_latency)
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

    pub async fn update_node_metrics(&self, node_id: usize, metrics: NodeMetrics) {
        let mut node_metrics = self.node_metrics.lock().unwrap();
        node_metrics.insert(node_id, metrics);
    }

    pub async fn start_heartbeat(&self) {
        let node_metrics = Arc::clone(&self.node_metrics);
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

use rand::Rng;
use std::collections::HashMap;

pub struct ResourceScheduler {
    pub historical_data: VecDeque<(usize, NodeMetrics)>,
}

impl ResourceScheduler {
    pub fn new() -> Self {
        ResourceScheduler {
            historical_data: VecDeque::new(),
        }
    }

    pub fn allocate(&self, resources: HashMap<usize, Resource>, node_metrics: HashMap<usize, NodeMetrics>, required: Resource, task_priority: f64, current_node: usize, network_latency: &NetworkLatency) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        for (node_id, resource) in resources.iter() {
            if resource.cpu >= required.cpu && resource.gpu >= required.gpu && resource.memory >= required.memory {
                if let Some(metrics) = node_metrics.get(node_id) {
                    let latency = network_latency.latencies.get(&(current_node, *node_id)).cloned().unwrap_or(f64::MAX);
                    let score = self.calculate_score(metrics, task_priority, latency);
                    heap.push(Reverse((score, *node_id)));
                }
            }
        }

        heap.pop().map(|Reverse((_, node_id))| node_id)
    }

    fn calculate_score(&self, metrics: &NodeMetrics, task_priority: f64, latency: f64) -> f64 {
        // Placeholder for a more sophisticated scoring function
        // This could be replaced with a machine learning model
        metrics.load * 0.5 + metrics.latency * 0.3 + metrics.reliability * 0.2 + task_priority - latency
    }

    pub fn update_historical_data(&mut self, node_id: usize, metrics: NodeMetrics) {
        self.historical_data.push_back((node_id, metrics));
        if self.historical_data.len() > 1000 {
            self.historical_data.pop_front();
        }
    }

    pub fn predict_node_performance(&self, node_id: usize) -> NodeMetrics {
        // Placeholder for a machine learning model to predict node performance
        // This could be replaced with an actual model
        let mut rng = rand::thread_rng();
        NodeMetrics {
            load: rng.gen_range(0.0..1.0),
            latency: rng.gen_range(0.0..1.0),
            reliability: rng.gen_range(0.0..1.0),
        }
    }
}

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
