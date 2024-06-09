# QUP State

This section covers the QUPState struct and its associated methods.

## Overview

The QUPState struct provides methods for managing the state of the QUP module, including block height, timestamp, and validator set.

## Methods

- `new() -> Self`
- `get_block_height(&self) -> u64`
- `get_block_timestamp(&self) -> u64`
- `get_validator_set(&self) -> Vec<Validator>`
