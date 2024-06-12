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

        // Emit a cross-chain message to call the ERC20 function on Ethereum
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

        // Store the result in the contract's state
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
