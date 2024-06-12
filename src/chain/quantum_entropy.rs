
impl QuantumResistantConnectionManager {
    pub fn new() -> Self {
        let connections = Arc::new(QuantumResistantConnection::new());
        Self { connections }
    }

    pub async fn establish_connection(&self, node_id: &str) -> Result<Keypair, QuantumEntropyError> {
        let connection = self.connections.clone();
        connection.establish(node_id).await
    }

    pub async fn close_connection(&self, node_id: &str) -> Result<(), QuantumEntropyError> {
        let connection = self.connections.clone();
        connection.close(node_id).await
    }
}

pub async fn establish_quantum_resistant_connection(
    manager: &QuantumResistantConnectionManager,
    node_id: &str,
) -> Result<Keypair, QuantumEntropyError> {
    info!("Establishing quantum-resistant connection for node: {}", node_id);
    let keypair = manager.establish_connection(node_id).await?;
    debug!("Quantum-resistant connection established successfully");
    Ok(keypair)
}

pub async fn close_quantum_resistant_connection(
    manager: &QuantumResistantConnectionManager,
    node_id: &str,
) -> Result<(), QuantumEntropyError> {
    info!("Closing quantum-resistant connection for node: {}", node_id);
    manager.close_connection(node_id).await?;
    debug!("Quantum-resistant connection closed successfully");
    Ok(())
}
