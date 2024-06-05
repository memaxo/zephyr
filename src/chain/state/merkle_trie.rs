use crate::crypto::post_quantum::{sign, verify, PostQuantumSignature};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MerkleTrieError {
    #[error("Trie node not found")]
    NodeNotFound,
    #[error("Invalid Merkle proof")]
    InvalidProof,
    #[error("Post-quantum signature verification failed")]
    SignatureVerificationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrieNode {
    pub key: Arc<[u8]>,
    pub value: Option<Arc<[u8]>>,
    pub children: HashMap<u8, Arc<TrieNode>>,
    pub signature: Option<PostQuantumSignature>,
}

impl TrieNode {
    pub fn new(key: Arc<[u8]>, value: Option<Arc<[u8]>>) -> Self {
        TrieNode {
            key,
            value,
            children: HashMap::new(),
            signature: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.key);
        if let Some(value) = &self.value {
            hasher.update(value);
        }
        hasher.finalize().to_vec()
    }

    pub fn sign(&mut self, private_key: &[u8]) -> Result<(), MerkleTrieError> {
        let hash = self.hash();
        let signature = sign(&hash, private_key)?;
        self.signature = Some(signature);
        Ok(())
    }

    pub fn verify_signature(&self, public_key: &[u8]) -> Result<bool, MerkleTrieError> {
        if let Some(signature) = &self.signature {
            let hash = self.hash();
            verify(&hash, signature, public_key)
                .map_err(|_| MerkleTrieError::SignatureVerificationFailed)
        } else {
            Ok(false)
        }
    }
}

pub struct MerkleTrie {
    root: Option<Arc<TrieNode>>,
}

impl MerkleTrie {
    pub fn new() -> Self {
        MerkleTrie { root: None }
    }

    pub fn insert(&mut self, key: &[u8], value: &[u8]) {
        let mut current_node = self
            .root
            .get_or_insert_with(|| Arc::new(TrieNode::new(Arc::from(vec![]), None)));
        let mut key_iter = key.iter();

        loop {
            if let Some(&byte) = key_iter.next() {
                let node = current_node
                    .children
                    .entry(byte)
                    .or_insert_with(|| Arc::new(TrieNode::new(Arc::from(vec![byte]), None)))
                    .clone();
                current_node = node;
            } else {
                current_node.value = Some(Arc::from(value));
                break;
            }
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Arc<[u8]>> {
        let mut current_node = self.root.as_ref()?;
        let mut key_iter = key.iter();

        loop {
            if let Some(&byte) = key_iter.next() {
                current_node = current_node.children.get(&byte)?;
            } else {
                return current_node.value.clone();
            }
        }
    }

    pub fn remove(&mut self, key: &[u8]) -> Result<(), MerkleTrieError> {
        self.remove_recursive(key, &mut self.root)
    }

    fn remove_recursive(
        &mut self,
        key: &[u8],
        node: &mut Option<Arc<TrieNode>>,
    ) -> Result<(), MerkleTrieError> {
        let mut current_node = Arc::make_mut(node.as_mut().ok_or(MerkleTrieError::NodeNotFound)?);
        let mut key_iter = key.iter();

        if let Some(&byte) = key_iter.next() {
            self.remove_recursive(
                key_iter.as_slice(),
                &mut current_node.children.get_mut(&byte).map(Arc::clone),
            )?;

            if current_node.children[&byte].is_leaf()
                && current_node.children[&byte].value.is_none()
            {
                current_node.children.remove(&byte);
            }
        } else {
            current_node.value = None;
        }

        Ok(())
    }

    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash())
    }

    pub fn generate_proof(&self, key: &[u8]) -> Option<Vec<Vec<u8>>> {
        let mut proof = Vec::new();
        self.generate_proof_recursive(key, self.root.as_ref(), &mut proof)?;
        Some(proof)
    }

    fn generate_proof_recursive(
        &self,
        key: &[u8],
        node: Option<&Arc<TrieNode>>,
        proof: &mut Vec<Vec<u8>>,
    ) -> Option<()> {
        let current_node = node?;
        proof.push(current_node.hash());

        if let Some(&byte) = key.first() {
            self.generate_proof_recursive(&key[1..], current_node.children.get(&byte), proof)?;
        }

        Some(())
    }

    pub fn verify_proof(
        &self,
        root_hash: &[u8],
        key: &[u8],
        value: &[u8],
        proof: &[Vec<u8>],
    ) -> Result<bool, MerkleTrieError> {
        let mut key_iter = key.iter();
        let mut hash = root_hash.to_vec();

        for p in proof {
            let mut hasher = Sha3_256::new();
            hasher.update(&hash);
            hasher.update(p);
            hash = hasher.finalize().to_vec();

            if let Some(&byte) = key_iter.next() {
                if byte != p[0] {
                    return Ok(false);
                }
            } else {
                return Ok(p == value);
            }
        }

        Ok(hash == self.root_hash().ok_or(MerkleTrieError::NodeNotFound)?)
    }

    pub fn sign(&mut self, private_key: &[u8]) -> Result<(), MerkleTrieError> {
        if let Some(root) = &mut self.root {
            sign_recursive(root, private_key)?;
        }
        Ok(())
    }

    pub fn verify_signature(&self, public_key: &[u8]) -> Result<bool, MerkleTrieError> {
        if let Some(root) = &self.root {
            verify_signature_recursive(root, public_key)
        } else {
            Ok(true)
        }
    }
}

fn sign_recursive(node: &mut Arc<TrieNode>, private_key: &[u8]) -> Result<(), MerkleTrieError> {
    let mut node = Arc::make_mut(node);
    node.sign(private_key)?;

    for child in node.children.values_mut() {
        sign_recursive(child, private_key)?;
    }

    Ok(())
}

fn verify_signature_recursive(
    node: &Arc<TrieNode>,
    public_key: &[u8],
) -> Result<bool, MerkleTrieError> {
    if !node.verify_signature(public_key)? {
        return Ok(false);
    }

    for child in node.children.values() {
        if !verify_signature_recursive(child, public_key)? {
            return Ok(false);
        }
    }

    Ok(true)
}
