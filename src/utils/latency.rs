use std::net::UdpSocket;
use std::time::{Duration, Instant};

pub struct Latency {
    pub node_id: String,
    pub latency: Duration,
}

pub fn measure_latency(target: &str) -> Result<Duration, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    socket.set_read_timeout(Some(Duration::from_secs(1))).map_err(|e| e.to_string())?;
    socket.set_nonblocking(true).map_err(|e| e.to_string())?;

    let start = Instant::now();
    socket.send_to(&[0], target).map_err(|e| e.to_string())?;

    let mut buf = [0; 1];
    match socket.recv_from(&mut buf) {
        Ok(_) => Ok(start.elapsed()),
        Err(_) => Err("Failed to receive response".to_string()),
    }
}

pub fn ping_nodes(nodes: Vec<String>) -> Vec<Latency> {
    nodes.into_iter().filter_map(|node| {
        match measure_latency(&node) {
            Ok(latency) => Some(Latency { node_id: node, latency }),
            Err(_) => None,
        }
    }).collect()
}
