# Advanced Usage

This section provides advanced usage examples for the QUP module.

## Example 1: Integrating QUP with a Network

```rust
use qup::integration::QUPIntegration;
use qup::network::Network;
use qup::storage::QUPStorage;
use qup::crypto::QUPCrypto;
use qup::state::QUPState;
use qup::smart_contract::SmartContractExecutor;

let network = Network::new();
let storage = QUPStorage::new();
let qup_crypto = QUPCrypto::new();
let qup_state = QUPState::new();
let smart_contract_executor = SmartContractExecutor::new();

let qup_integration = QUPIntegration::new(
    network,
    storage,
    smart_contract_executor,
    qup_crypto,
    qup_state,
);
```

## Example 2: Customizing QUP Config

```rust
use qup::config::QUPConfig;

let qup_config = QUPConfig {
    consensus_algorithm: "Hybrid".to_string(),
    useful_work_difficulty: 10,
    voting_threshold: 5,
    validator_reward_ratio: 0.7,
    delegator_reward_ratio: 0.3,
};
```
