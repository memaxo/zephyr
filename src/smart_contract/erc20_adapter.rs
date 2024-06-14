use crate::smart_contract::types::{Value, ExecutionContext, CrossChainMessage};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ERC20AdapterConfig {
    pub zephyr_chain_id: String,
    pub ethereum_chain_id: String,
    pub erc20_token_address: String,
}

pub struct ERC20Adapter {
    config: ERC20AdapterConfig,
}

impl ERC20Adapter {
    pub fn new(config: ERC20AdapterConfig) -> Self {
        ERC20Adapter { config }
    }

    pub fn call_erc20_function(&self, context: &mut ExecutionContext, function: &str, args: &[Value]) -> Result<(), String> {
        // Serialize the function name and arguments
        let payload = serde_json::json!({
            "function": function,
            "args": args,
        });

        // Create and verify cross-chain message
        let message = CrossChainMessage {
            source_chain: self.config.zephyr_chain_id.clone(),
            destination_chain: self.config.ethereum_chain_id.clone(),
            packet_data: IBCPacketData {
                sequence: 0,
                timeout_height: 0,
                timeout_timestamp: 0,
                source_port: "".to_string(),
                source_channel: "".to_string(),
                dest_port: "".to_string(),
                dest_channel: "".to_string(),
                data: vec![],
            },
            timestamp: 0,
            signatures: vec![],
            public_key: context.get_public_key()?,
            signature: vec![],
        };

        if !message.verify_signature() {
            return Err("Invalid message signature".to_string());
        }
        let message = CrossChainMessage {
            source_chain: self.config.zephyr_chain_id.clone(),
            destination_chain: self.config.ethereum_chain_id.clone(),
            action: "call_erc20_function".to_string(),
            payload: serde_json::to_string(&payload)?,
        };
        context.emit_cross_chain_message(message)?;

        info!("Called ERC20 function {} with args {:?}", function, args);

        Ok(())
    }

    pub fn handle_erc20_result(&self, context: &mut ExecutionContext, payload: &str) -> Result<(), String> {
        // Parse the payload to get the result
        let payload: serde_json::Value = serde_json::from_str(payload)?;
        let result = payload["result"].clone();

        // Verify the signature of the result
        let signature = payload["signature"].as_str().ok_or("Missing signature in payload")?;
        let public_key = self.get_erc20_public_key()?;
        if !self.verify_signature(&result, signature, &public_key) {
            return Err("Invalid result signature".to_string());
        }

        // Store the verified result in the contract's state
        context.set_state("erc20_result".to_string(), result)?;

        info!("Received ERC20 result: {:?}", result);

        Ok(())
    }

    pub fn handle_cross_chain_message(&self, context: &mut ExecutionContext, message: CrossChainMessage) -> Result<(), String> {
        if message.destination_chain != self.config.zephyr_chain_id {
            return Err("Invalid destination chain".to_string());
        }

        match message.action.as_str() {
            "erc20_result" => self.handle_erc20_result(context, &message.payload),
            _ => Err(format!("Invalid cross-chain action: {}", message.action)),
        }
    }
}
impl ERC20Adapter {
    // Other methods...

    fn verify_signature(&self, message: &serde_json::Value, signature: &str, public_key: &str) -> bool {
        use pqcrypto_dilithium::dilithium2::{self, PublicKey, Signature};

        // Deserialize the public key
        let public_key_bytes = match hex::decode(public_key) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let public_key = match PublicKey::from_bytes(&public_key_bytes) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        // Deserialize the signature
        let signature_bytes = match hex::decode(signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let signature = match Signature::from_bytes(&signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        // Serialize the message
        let message_bytes = match serde_json::to_vec(message) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        // Verify the signature
        dilithium2::verify(&message_bytes, &signature, &public_key).is_ok()
    }
}
impl ERC20Adapter {
    // Other methods...

    fn get_erc20_public_key(&self) -> Result<String, String> {
        // Placeholder for actual logic to retrieve the public key
        // This could be stored in the contract's state or fetched from an external source
        Ok("public_key_placeholder".to_string())
    }
}
