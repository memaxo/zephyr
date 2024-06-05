#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_protocol_message_serialization() {
        let message = ProtocolMessage::Ping;
        let serialized = message.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[tokio::test]
    async fn test_classical_key_exchange() {
        let message = ProtocolMessage::ClassicalKeyExchange {
            public_key: vec![1, 2, 3, 4],
        };
        let serialized = message.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[tokio::test]
    async fn test_classical_key_exchange_response() {
        let message = ProtocolMessage::ClassicalKeyExchangeResponse {
            public_key: vec![5, 6, 7, 8],
        };
        let serialized = message.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[tokio::test]
    async fn test_quantum_key_exchange() {
        let message = ProtocolMessage::QuantumKeyExchange {
            public_key: vec![9, 10, 11, 12],
        };
        let serialized = message.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[tokio::test]
    async fn test_quantum_key_exchange_response() {
        let message = ProtocolMessage::QuantumKeyExchangeResponse {
            public_key: vec![13, 14, 15, 16],
        };
        let serialized = message.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[tokio::test]
    async fn test_state_sync_request() {
        let message = ProtocolMessage::StateSyncRequest { shard_id: 1 };
        let serialized = message.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[tokio::test]
    async fn test_state_sync_response() {
        let message = ProtocolMessage::StateSyncResponse {
            shard_id: 1,
            state: vec![17, 18, 19, 20],
        };
        let serialized = message.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}
