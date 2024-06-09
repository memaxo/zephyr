# QUP Utils

This section covers utility functions used in the QUP module.

## Overview

The QUPUtils module provides various utility functions for common operations in the QUP module.

## Functions

- `hash(data: &[u8]) -> Vec<u8>`
- `sign(data: &[u8], private_key: &[u8]) -> Vec<u8>`
- `verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool`
