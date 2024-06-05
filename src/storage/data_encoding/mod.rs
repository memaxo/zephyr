/// The `data_encoding` module provides functionality for encoding and decoding
/// various types of data within the Zephyr blockchain project. This includes
/// encoding transactions, states, blocks, and smart contracts.

pub mod block_encoder;
pub mod transaction_encoder;
pub mod state_encoder;
pub mod smart_contract_encoder;

pub use block_encoder::{encode_block, decode_block};
pub use transaction_encoder::{TransactionEncoder, encode_transaction, decode_transaction};
pub use state_encoder::{StateEncoder, encode_state, decode_state};
pub use smart_contract_encoder::{SmartContractEncoder, encode_smart_contract, decode_smart_contract};
