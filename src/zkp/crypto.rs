use pqcrypto_dilithium::dilithium2::{keypair as dilithium_keypair, sign as dilithium_sign, verify as dilithium_verify, PublicKey as DilithiumPublicKey, SecretKey as DilithiumSecretKey, Signature as DilithiumSignature};
use pqcrypto_kyber::kyber1024::{PublicKey as KyberPublicKey, SecretKey as KyberSecretKey, Ciphertext as KyberCiphertext};
use rand::rngs::OsRng;
use num_bigint::BigUint;
use zeroize::Zeroize;
use pqcrypto_dilithium::dilithium2::{keypair as dilithium_keypair, sign as dilithium_sign, verify as dilithium_verify, PublicKey as DilithiumPublicKey, SecretKey as DilithiumSecretKey, Signature as DilithiumSignature};
use pqcrypto_kyber::kyber1024::{keypair as kyber_keypair, encapsulate as kyber_encapsulate, decapsulate as kyber_decapsulate, PublicKey as KyberPublicKey, SecretKey as KyberSecretKey, Ciphertext as KyberCiphertext};
use rand::rngs::OsRng;
use num_bigint::BigUint;
use zeroize::Zeroize;

pub trait Hasher {
    fn new() -> Self;
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> FieldElement;
}

pub struct QuantumResistantMerkleTree {
    levels: Vec<Vec<FieldElement>>,
    dilithium_public_key: DilithiumPublicKey,
    dilithium_secret_key: DilithiumSecretKey,
}

impl QuantumResistantMerkleTree {
    pub fn new(leaves: &[FieldElement]) -> Self {
        let mut levels = vec![leaves.to_vec()];

        while levels.last().unwrap().len() > 1 {
            let level = Self::build_level(levels.last().unwrap());
            levels.push(level);
        }

        let (dilithium_public_key, dilithium_secret_key) = dilithium_keypair();

        QuantumResistantMerkleTree {
            levels,
            dilithium_public_key,
            dilithium_secret_key,
        }
    }

    fn build_level(prev_level: &[FieldElement]) -> Vec<FieldElement> {
        let mut level = Vec::new();

        for i in (0..prev_level.len()).step_by(2) {
            let mut hasher = Sha256::new();
            hasher.update(&prev_level[i].to_bytes());
            if i + 1 < prev_level.len() {
                hasher.update(&prev_level[i + 1].to_bytes());
            }
            let hash = FieldElement::new(BigUint::from_bytes_le(&hasher.finalize()), &BigUint::from(1u32) << 256);
            level.push(hash);
        }

        level
    }

    pub fn root(&self) -> FieldElement {
        self.levels.last().unwrap()[0].clone()
    }

    pub fn proof(&self, index: usize) -> (Vec<FieldElement>, DilithiumSignature) {
        let mut proof = Vec::new();
        let mut index = index;

        for level in &self.levels[..self.levels.len() - 1] {
            let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
            if sibling_index < level.len() {
                proof.push(level[sibling_index].clone());
            }
            index /= 2;
        }

        let message = proof.iter().map(|p| p.to_bytes()).flatten().collect::<Vec<u8>>();
        let dilithium_signature = dilithium_sign(&message, &self.dilithium_secret_key);
        (proof, dilithium_signature)
    }
}

pub fn verify_quantum_merkle_proof(
    root: &FieldElement,
    proof: &[FieldElement],
    dilithium_signature: &DilithiumSignature,
    dilithium_public_key: &DilithiumPublicKey,
    leaf: &FieldElement,
    index: usize,
) -> bool {
    let mut current = leaf.clone();
    let mut index = index;

    for sibling in proof {
        let mut hasher = Sha256::new();
        if index % 2 == 0 {
            hasher.update(&current.to_bytes());
            hasher.update(&sibling.to_bytes());
        } else {
            hasher.update(&sibling.to_bytes());
            hasher.update(&current.to_bytes());
        }
        current = FieldElement::new(BigUint::from_bytes_le(&hasher.finalize()), &BigUint::from(1u32) << 256);
        index /= 2;
    }

    if &current != root {
        return false;
    }

    let message = proof.iter().map(|p| p.to_bytes()).flatten().collect::<Vec<u8>>();
    dilithium_verify(&message, dilithium_signature, dilithium_public_key).is_ok()
}
