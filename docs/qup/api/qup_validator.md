# QUP Validator

This section covers the QUPValidator struct and its associated methods.

## Overview

The QUPValidator struct provides methods for managing validators in the QUP module, including signing and verifying blocks.

## Methods

- `new() -> Self`
- `sign(&self, data: &[u8]) -> Vec<u8>`
- `verify(&self, data: &[u8], signature: &[u8]) -> bool`
