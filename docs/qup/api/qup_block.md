# QUP Block

This section covers the QUPBlock struct and its associated methods.

## Overview

The QUPBlock struct represents a block in the QUP blockchain. It includes methods for creating, hashing, signing, and validating blocks.

## Methods

- `new(height: u64, timestamp: u64, prev_block_hash: Hash, transactions: Vec<Transaction>, useful_work_problem: Option<UsefulWorkProblem>, useful_work_solution: Option<UsefulWorkSolution>, history_proof: Vec<Hash>, validator: &QUPValidator) -> Self`
- `hash(&self) -> Hash`
- `sign(&mut self, validator: &QUPValidator)`
- `verify_signature(&self, validator: &QUPValidator) -> bool`
- `validate(&self, state: &QUPState, config: &QUPConfig) -> Result<(), Error>`
