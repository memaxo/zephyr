use crate::smart_contract::types::{Value, ExecutionContext};
use crate::smart_contract::erc20_wrapper::ERC20WrapperContract;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DEXConfig {
    pub zephyr_asset_denom: String,
    pub erc20_token_address: String,
    pub erc20_token_decimals: u8,
}

pub struct DEXContract {
    config: DEXConfig,
    erc20_wrapper: ERC20WrapperContract,
}

impl DEXContract {
    pub fn new(config: DEXConfig) -> Self {
        let erc20_wrapper_config = ERC20WrapperConfig {
            zephyr_asset_denom: config.zephyr_asset_denom.clone(),
            erc20_token_address: config.erc20_token_address.clone(),
            erc20_token_decimals: config.erc20_token_decimals,
        };
        let erc20_wrapper = ERC20WrapperContract::new(erc20_wrapper_config);

        DEXContract {
            config,
            erc20_wrapper,
        }
    }

    pub fn swap_zephyr_to_erc20(&self, context: &mut ExecutionContext, amount: u64) -> Result<(), String> {
        // Deduct Zephyr assets from the sender's balance
        let sender = context.get_sender()?;
        context.transfer_assets(&sender, "system", self.config.zephyr_asset_denom.clone(), amount)?;

        // Deposit the assets into the ERC20 wrapper contract
        self.erc20_wrapper.deposit(context, amount)?;

        // Mint the corresponding ERC20 tokens to the sender
        self.erc20_wrapper.mint_tokens(context, sender, amount)?;

        info!("Swapped {} {} assets for {} ERC20 tokens", amount, self.config.zephyr_asset_denom, amount);

        Ok(())
    }

    pub fn swap_erc20_to_zephyr(&self, context: &mut ExecutionContext, amount: u64) -> Result<(), String> {
        // Burn the ERC20 tokens from the sender
        let sender = context.get_sender()?;
        self.erc20_wrapper.burn_tokens(context, sender, amount)?;

        // Withdraw the assets from the ERC20 wrapper contract
        self.erc20_wrapper.withdraw(context, amount)?;

        // Transfer the Zephyr assets to the sender
        context.transfer_assets("system", &sender, self.config.zephyr_asset_denom.clone(), amount)?;

        info!("Swapped {} ERC20 tokens for {} {} assets", amount, amount, self.config.zephyr_asset_denom);

        Ok(())
    }
}
