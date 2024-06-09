# QUP Delegator

This section covers the QUPDelegator struct and its associated methods.

## Overview

The QUPDelegator struct provides methods for managing delegations in the QUP module.

## Methods

- `new() -> Self`
- `delegate(&self, delegator: &str, validator: &str, amount: u64) -> Result<(), DelegationError>`
- `undelegate(&self, delegator: &str, validator: &str) -> Result<(), DelegationError>`
- `get_delegations(&self, delegator: &str) -> Vec<Delegation>`
