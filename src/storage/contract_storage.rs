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

    pub fn save_contract(&self, contract: &SmartContract) -> Result<(), ContractStorageError> {
        let contract_id = contract.id().to_string();
        let data = serde_json::to_vec(contract)
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;
        self.db.put(&contract_id, &data)
            .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))
    }

    pub fn load_contract(&self, contract_id: &str) -> Result<SmartContract, ContractStorageError> {
        match self.db.get(contract_id) {
            Some(data) => {
                let contract: SmartContract = serde_json::from_slice(&data)
                    .map_err(|e| ContractStorageError::DatabaseError(e.to_string()))?;
                Ok(contract)
            }
            None => Err(ContractStorageError::ContractNotFound(contract_id.to_string())),
        }
    }
}
