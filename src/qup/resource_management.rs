use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Resource {
    pub cpu: usize,
    pub gpu: usize,
    pub memory: usize,
}

pub struct ResourceManager {
    resources: Arc<Mutex<HashMap<usize, Resource>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_node(&self, node_id: usize, resource: Resource) {
        let mut resources = self.resources.lock().unwrap();
        resources.insert(node_id, resource);
    }

    pub fn remove_node(&self, node_id: usize) {
        let mut resources = self.resources.lock().unwrap();
        resources.remove(&node_id);
    }

    pub fn allocate_resources(&self, required: Resource) -> Option<usize> {
        let resources = self.resources.lock().unwrap();
        for (node_id, resource) in resources.iter() {
            if resource.cpu >= required.cpu && resource.gpu >= required.gpu && resource.memory >= required.memory {
                return Some(*node_id);
            }
        }
        None
    }

    pub fn update_resource(&self, node_id: usize, resource: Resource) {
        let mut resources = self.resources.lock().unwrap();
        if let Some(node_resource) = resources.get_mut(&node_id) {
            node_resource.cpu = resource.cpu;
            node_resource.gpu = resource.gpu;
            node_resource.memory = resource.memory;
        }
    }

    pub fn get_resources(&self) -> HashMap<usize, Resource> {
        let resources = self.resources.lock().unwrap();
        resources.clone()
    }
}
