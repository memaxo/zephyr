use crate::zkp_crate::math::FieldElement;
use pqcrypto_picnic::picnic_l1_fs::Hasher as PicnicHasher;
use crystals_dilithium::{dilithium2 as Dilithium, Keypair, PublicKey, Signature};
use rand::rngs::OsRng;
use num_bigint::BigUint;

pub trait Hasher {
    fn new() -> Self;
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> FieldElement;
}

pub struct QuantumResistantHasher(PicnicHasher);

impl Hasher for QuantumResistantHasher {
    fn new() -> Self {
        QuantumResistantHasher(PicnicHasher::new())
    }

    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    fn finalize(self) -> FieldElement {
        let digest = self.0.finalize();
        let bytes = digest.as_slice();
        let value = BigUint::from_bytes_le(bytes);
        FieldElement::new(value, &BigUint::from(1u32) << 256)
    }
}

pub struct QuantumResistantMerkleTree {
    levels: Vec<Vec<FieldElement>>,
    public_key: PublicKey,
    keypair: Keypair, // Add this line to store the Keypair
}

impl QuantumResistantMerkleTree {
    pub fn new(leaves: &[FieldElement]) -> Self {
        let mut levels = vec![leaves.to_vec()];

        while levels.last().unwrap().len() > 1 {
            let level = Self::build_level(levels.last().unwrap());
            levels.push(level);
        }

        let keypair = Keypair::generate(&mut OsRng);
        let public_key = keypair.public;

        QuantumResistantMerkleTree { levels, public_key, keypair } // Adjust this line
    }

    fn build_level(prev_level: &[FieldElement]) -> Vec<FieldElement> {
        let mut level = Vec::new();

        for i in (0..prev_level.len()).step_by(2) {
            let mut hasher = QuantumResistantHasher::new();
            hasher.update(&prev_level[i].to_bytes());
            if i + 1 < prev_level.len() {
                hasher.update(&prev_level[i + 1].to_bytes());
            }
            let hash = hasher.finalize();
            level.push(hash);
        }

        level
    }

    pub fn root(&self) -> FieldElement {
        self.levels.last().unwrap()[0].clone()
    }

    pub fn proof(&self, index: usize) -> (Vec<FieldElement>, Signature) {
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
        let signature = self.keypair.sign(&message); 

        (proof, signature)
    }
}

pub fn verify_quantum_merkle_proof(
    root: &FieldElement,
    proof: &[FieldElement],
    signature: &Signature,
    public_key: &PublicKey,
    leaf: &FieldElement,
    index: usize,
) -> bool {
    let mut current = leaf.clone();
    let mut index = index;

    for sibling in proof {
        let mut hasher = Blake3bHasher::new();
        if index % 2 == 0 {
            hasher.update(&current.to_bytes());
            hasher.update(&sibling.to_bytes());
        } else {
            hasher.update(&sibling.to_bytes());
            hasher.update(&current.to_bytes());
        }
        current = hasher.finalize();
        index /= 2;
    }

    if &current != root {
        return false;
    }

    let message = proof.iter().map(|p| p.to_bytes()).flatten().collect::<Vec<u8>>();
    Dilithium::verify(&message, signature, public_key)
}
