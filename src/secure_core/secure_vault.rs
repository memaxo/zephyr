use std::collections::HashMap;
pub struct SecureVault {
    kyber_keys: HashMap<String, (KyberPublicKey, KyberSecretKey)>,
    mceliece_keys: HashMap<String, (McEliecePublicKey, McElieceSecretKey)>,
    hqc_keys: HashMap<String, (HQCPublicKey, HQCSecretKey)>,
    dilithium_keys: HashMap<String, (DilithiumPublicKey, DilithiumSecretKey)>,
    falcon_keys: HashMap<String, (FalconPublicKey, FalconSecretKey)>,
    sphincsplus_keys: HashMap<String, (SPHINCSPlusPublicKey, SPHINCSPlusSecretKey)>,
}

impl SecureVault {
    pub fn new() -> Self {
        SecureVault {
            kyber_keys: HashMap::new(),
            mceliece_keys: HashMap::new(),
            hqc_keys: HashMap::new(),
            dilithium_keys: HashMap::new(),
            falcon_keys: HashMap::new(),
            sphincsplus_keys: HashMap::new(),
        }
    }

    pub fn store_kyber_keys(&mut self, key_id: &str, public_key: KyberPublicKey, secret_key: KyberSecretKey) {
        self.kyber_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    pub fn store_mceliece_keys(&mut self, key_id: &str, public_key: McEliecePublicKey, secret_key: McElieceSecretKey) {
        self.mceliece_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    pub fn store_hqc_keys(&mut self, key_id: &str, public_key: HQCPublicKey, secret_key: HQCSecretKey) {
        self.hqc_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    pub fn store_dilithium_keys(&mut self, key_id: &str, public_key: DilithiumPublicKey, secret_key: DilithiumSecretKey) {
        self.dilithium_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    pub fn store_falcon_keys(&mut self, key_id: &str, public_key: FalconPublicKey, secret_key: FalconSecretKey) {
        self.falcon_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    pub fn store_sphincsplus_keys(&mut self, key_id: &str, public_key: SPHINCSPlusPublicKey, secret_key: SPHINCSPlusSecretKey) {
        self.sphincsplus_keys.insert(key_id.to_string(), (public_key, secret_key));
    }

    // Add methods to retrieve keys by key_id
    // ...
}
