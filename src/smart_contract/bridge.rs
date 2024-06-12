use crate::smart_contract::types::{Value, ExecutionContext, CrossChainMessage};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub zephyr_chain_id: String,
    pub counterparty_chain_id: String,
    pub zephyr_asset_denom: String,
    pub counterparty_asset_denom: String,
}

pub struct BridgeContract {
    config: BridgeConfig,
}

impl BridgeContract {
    pub fn new(config: BridgeConfig) -> Self {
        BridgeContract { config }
    }

    pub fn lock_assets(&self, context: &mut ExecutionContext, amount: u64) -> Result<(), String> {
        // Deduct assets from the sender's balance
        let sender = context.get_sender()?;
        context.transfer_assets(&sender, "system", self.config.zephyr_asset_denom.clone(), amount)?;

        // Emit a cross-chain message to lock the assets on the counterparty chain
        let message = CrossChainMessage {
            source_chain: self.config.zephyr_chain_id.clone(),
            destination_chain: self.config.counterparty_chain_id.clone(),
            action: "lock".to_string(),
            payload: format!("{{\"amount\": {}, \"denom\": \"{}\"}}", amount, self.config.counterparty_asset_denom),
        };
        context.emit_cross_chain_message(message)?;

        info!("Locked {} {} assets for transfer to {}", amount, self.config.zephyr_asset_denom, self.config.counterparty_chain_id);

        Ok(())
    }

    pub fn unlock_assets(&self, context: &mut ExecutionContext, payload: &str) -> Result<(), String> {
        // Parse the payload to get the amount and recipient
        let payload: serde_json::Value = serde_json::from_str(payload)?;
        let amount = payload["amount"].as_u64().ok_or("Invalid amount in payload")?;
        let recipient = payload["recipient"].as_str().ok_or("Invalid recipient in payload")?.to_string();

        // Mint new assets to the recipient
        context.mint_assets(&recipient, self.config.zephyr_asset_denom.clone(), amount)?;

        info!("Unlocked {} {} assets received from {}", amount, self.config.zephyr_asset_denom, self.config.counterparty_chain_id);

        Ok(())
    }

    pub fn handle_cross_chain_message(&self, context: &mut ExecutionContext, message: CrossChainMessage) -> Result<(), String> {
        if message.destination_chain != self.config.zephyr_chain_id {
            return Err("Invalid destination chain".to_string());
        }

        match message.action.as_str() {
            "unlock" => self.unlock_assets(context, &message.payload),
            _ => Err(format!("Invalid cross-chain action: {}", message.action)),
        }
    }
}
