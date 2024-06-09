# HD Communication

This section covers the high-dimensional communication protocols used in the QUP module.

## Overview

The HDCommunication struct provides methods for encoding and decoding high-dimensional data for secure communication in the QUP module.

## Methods

- `new() -> Self`
- `encode(&self, data: &[u8]) -> Vec<f64>`
- `decode(&self, encoded_data: &[f64]) -> Vec<u8>`
