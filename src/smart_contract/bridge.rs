use crate::smart_contract::types::{Value, ExecutionContext, CrossChainMessage};
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::smart_contract::erc20_wrapper::ERC20WrapperContract;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub zephyr_chain_id: String,
    pub ethereum_chain_id: String,
    pub zephyr_asset_denom: String,
    pub erc20_token_address: String,
    pub erc20_token_decimals: u8,
}

pub struct BridgeContract {
    config: BridgeConfig,
    erc20_wrapper: ERC20WrapperContract,
}

lazy_static! {
    static ref REENTRANCY_GUARD: Mutex<()> = Mutex::new(());
}

impl BridgeContract {
    pub fn new(config: BridgeConfig) -> Self {
        let erc20_wrapper_config = ERC20WrapperConfig {
            zephyr_asset_denom: config.zephyr_asset_denom.clone(),
            erc20_token_address: config.erc20_token_address.clone(),
            erc20_token_decimals: config.erc20_token_decimals,
        };
        let erc20_wrapper = ERC20WrapperContract::new(erc20_wrapper_config);

        BridgeContract {
            config,
            erc20_wrapper,
        }
    }

    pub fn lock_assets(&self, context: &mut ExecutionContext, amount: u64) -> Result<(), String> {
        let _guard = REENTRANCY_GUARD.lock().unwrap(); // Acquire lock

        // Deduct assets from the sender's balance
        let sender = context.get_sender()?;
        context.transfer_assets(&sender, "system", self.config.zephyr_asset_denom.clone(), amount)?;

        // Deposit the assets into the ERC20 wrapper contract
        self.erc20_wrapper.deposit(context, amount)?;

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
            action: "mint".to_string(),
            payload: format!("{{\"amount\": {}, \"recipient\": \"{}\"}}", amount, context.get_sender()?),
        };
        context.emit_cross_chain_message(message)?;

        // Release the lock AFTER the message is sent
        drop(_guard); // Explicitly release the lock

        info!("Locked {} {} assets for transfer to {}", amount, self.config.zephyr_asset_denom, self.config.counterparty_chain_id);

        Ok(())
    }

    pub fn unlock_assets(&self, context: &mut ExecutionContext, payload: &str) -> Result<(), String> {
        // Parse the payload to get the amount and recipient
        let payload: serde_json::Value = serde_json::from_str(payload)?;
        let amount = payload["amount"].as_u64().ok_or("Invalid amount in payload")?;
        let recipient = payload["recipient"].as_str().ok_or("Invalid recipient in payload")?.to_string();

        // Withdraw the assets from the ERC20 wrapper contract
        self.erc20_wrapper.withdraw(context, amount)?;

        // Emit a cross-chain message to unlock the assets on Ethereum
        let message = CrossChainMessage {
            source_chain: self.config.zephyr_chain_id.clone(),
            destination_chain: self.config.ethereum_chain_id.clone(),
            action: "unlock".to_string(),
            payload: format!("{{\"amount\": {}, \"recipient\": \"{}\"}}", amount, recipient),
        };
        context.emit_cross_chain_message(message)?;

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
