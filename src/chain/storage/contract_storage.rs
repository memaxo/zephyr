use crate::chain::smart_contract::SmartContract;
use crate::chain::storage::database::Database;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractStorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Contract not found: {0}")]
    ContractNotFound(String),
}

pub struct ContractStorage {
    db: Box<dyn Database>,
}

impl ContractStorage {
    pub fn new(db: Box<dyn Database>) -> Self {
        ContractStorage { db }
    }

    pub fn deploy_contract(&self, contract: &SmartContract) -> Result<(), ContractStorageError> {
        let contract_key = format!("contract:{}", contract.id);
        let contract_data = serde_json::to_vec(contract)
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;

        self.db
            .put(contract_key.as_bytes(), &contract_data)
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn get_contract(&self, contract_id: &str) -> Result<Option<SmartContract>, ContractStorageError> {
        let contract_key = format!("contract:{}", contract_id);
        let contract_data = self
            .db
            .get(contract_key.as_bytes())
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;

        match contract_data {
            Some(data) => {
                let contract = serde_json::from_slice(&data)
                    .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;
                Ok(Some(contract))
            }
            None => Ok(None),
        }
    }

    pub fn update_contract(&self, contract: &SmartContract) -> Result<(), ContractStorageError> {
        let contract_key = format!("contract:{}", contract.id);
        let contract_data = serde_json::to_vec(contract)
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;

        self.db
            .put(contract_key.as_bytes(), &contract_data)
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn delete_contract(&self, contract_id: &str) -> Result<(), ContractStorageError> {
        let contract_key = format!("contract:{}", contract_id);
        self.db
            .delete(contract_key.as_bytes())
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn get_all_contracts(&self) -> Result<Vec<SmartContract>, ContractStorageError> {
        let contract_prefix = "contract:";
        let contracts = self
            .db
            .iterate_with_prefix(contract_prefix.as_bytes())
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();
        for (_, contract_data) in contracts {
            let contract = serde_json::from_slice(&contract_data)
                .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;
            result.push(contract);
        }

        Ok(result)
    }
}