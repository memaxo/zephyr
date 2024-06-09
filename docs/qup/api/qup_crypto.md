# QUP Crypto

This section covers the cryptographic functions used in the QUP module.

## Overview

The QUPCrypto struct provides methods for encryption, decryption, signing, and verifying signatures using post-quantum cryptographic algorithms.

## Methods

- `new() -> Self`
- `encrypt<P: Encrypt>(&self, data: &[u8], public_key: &P) -> Vec<u8>`
- `decrypt<S: Decrypt>(&self, ciphertext: &[u8], secret_key: &S) -> Vec<u8>`
- `sign<S: Sign>(&self, data: &[u8], secret_key: &S) -> Vec<u8>`
- `verify<P: Verify>(&self, data: &[u8], signature: &[u8], public_key: &P) -> bool`
