impl<P, S> KeyPair<P, S> {
    pub fn new(public_key: P, secret_key: S) -> Self {
        KeyPair { public_key, secret_key }
    }
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
