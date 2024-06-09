# Integration

This section provides examples of integrating the QUP module with other systems.

## Example 1: Integrating QUP with a Blockchain

```rust
use qup::block::QUPBlock;
use qup::crypto::QUPValidator;
use qup::state::QUPState;
use qup::config::QUPConfig;

let validator = QUPValidator::new();
let state = QUPState::new();
let config = QUPConfig::default();

let block = QUPBlock::new(
    1,
    1627846267,
    vec![],
    vec![],
    None,
    None,
    vec![],
    &validator,
);

block.sign(&validator);
assert!(block.verify_signature(&validator));
assert!(block.validate(&state, &config).is_ok());
```

## Example 2: Integrating QUP with a Smart Contract

```rust
use qup::smart_contract::SmartContractExecutor;
use qup::crypto::QUPCrypto;

let qup_crypto = QUPCrypto::new();
let smart_contract_executor = SmartContractExecutor::new();

let contract_code = r#"
    contract HelloWorld {
        function sayHello() public pure returns (string memory) {
            return "Hello, QUP!";
        }
    }
"#;

let compiled_contract = smart_contract_executor.compile(contract_code);
let contract_address = smart_contract_executor.deploy(&compiled_contract, &qup_crypto);
assert!(smart_contract_executor.call(&contract_address, "sayHello", &[]).is_ok());
```
