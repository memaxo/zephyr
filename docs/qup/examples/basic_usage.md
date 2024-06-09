# Basic Usage

This section provides basic usage examples for the QUP module.

## Example 1: Creating a QUP Block

```rust
use qup::block::QUPBlock;
use qup::crypto::QUPValidator;

let validator = QUPValidator::new();
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
```

## Example 2: Using QUP Crypto

```rust
use qup::crypto::QUPCrypto;

let qup_crypto = QUPCrypto::new();
let data = b"Hello, QUP!";
let signature = qup_crypto.sign(data, &qup_crypto.dilithium_keypair.secret_key);
assert!(qup_crypto.verify(data, &signature, &qup_crypto.dilithium_keypair.public_key));
```
