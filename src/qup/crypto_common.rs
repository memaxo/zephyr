pub struct KeyPair<P, S> {
    pub public_key: P,
    pub secret_key: S,
}

pub trait Encrypt {
    fn encrypt(&self, data: &[u8]) -> Vec<u8>;
}

pub trait Decrypt {
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8>;
}

pub trait Sign {
    fn sign(&self, data: &[u8]) -> Vec<u8>;
}

pub trait Verify {
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool;
}
