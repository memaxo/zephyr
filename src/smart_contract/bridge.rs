use crate::smart_contract::types::{Value, ExecutionContext, CrossChainMessage};
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
        // Deduct assets from the sender's balance
        let sender = context.get_sender()?;
        context.transfer_assets(&sender, "system", self.config.zephyr_asset_denom.clone(), amount)?;

        // Deposit the assets into the ERC20 wrapper contract
        self.erc20_wrapper.deposit(context, amount)?;

        // Emit a cross-chain message to mint the wrapped tokens on Ethereum
        let message = CrossChainMessage {
            source_chain: self.config.zephyr_chain_id.clone(),
            destination_chain: self.config.ethereum_chain_id.clone(),
            action: "mint".to_string(),
            payload: format!("{{\"amount\": {}, \"recipient\": \"{}\"}}", amount, context.get_sender()?),
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