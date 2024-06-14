use crate::smart_contract::types::{Value, ExecutionContext};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ERC20WrapperConfig {
    pub zephyr_asset_denom: String,
    pub erc20_token_address: String,
    pub erc20_token_decimals: u8,
}

pub struct ERC20WrapperContract {
    config: ERC20WrapperConfig,
    balances: HashMap<String, u64>,
}

impl ERC20WrapperContract {
    pub fn new(config: ERC20WrapperConfig) -> Self {
        ERC20WrapperContract {
            config,
            balances: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, context: &mut ExecutionContext, amount: u64) -> Result<(), String> {
        let sender = context.get_sender()?;

        // Transfer Zephyr assets from the sender to the contract
        context.transfer_assets(&sender, "system", self.config.zephyr_asset_denom.clone(), amount)?;

        // Mint corresponding ERC20 tokens to the sender
        self.mint_tokens(&sender, amount)?;

        info!("Deposited {} {} assets and minted {} ERC20 tokens for {}", amount, self.config.zephyr_asset_denom, amount, sender);

        Ok(())
    }

    pub fn withdraw(&mut self, context: &mut ExecutionContext, amount: u64) -> Result<(), String> {
        let sender = context.get_sender()?;

        // Burn ERC20 tokens from the sender
        self.burn_tokens(&sender, amount)?;

        // Transfer corresponding Zephyr assets from the contract to the sender
        context.transfer_assets("system", &sender, self.config.zephyr_asset_denom.clone(), amount)?;

        info!("Burned {} ERC20 tokens and withdrew {} {} assets for {}", amount, amount, self.config.zephyr_asset_denom, sender);

        Ok(())
    }

    fn mint_tokens(&mut self, recipient: &str, amount: u64) -> Result<(), String> {
        let balance = self.balances.entry(recipient.to_string()).or_insert(0);
        *balance = balance.checked_add(amount).ok_or("Integer overflow")?;
        Ok(())
    }

    fn burn_tokens(&mut self, sender: &str, amount: u64) -> Result<(), String> {
        let balance = self.balances.get_mut(sender).ok_or_else(|| format!("Insufficient balance for {}", sender))?;
        *balance = balance.checked_sub(amount).ok_or("Integer underflow")?;
        Ok(())
    }

    pub fn get_erc20_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
}
