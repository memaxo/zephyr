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

    pub fn get_kyber_keys(&self, key_id: &str) -> Option<&(KyberPublicKey, KyberSecretKey)> {
        self.kyber_keys.get(key_id)
    }

    pub fn get_mceliece_keys(&self, key_id: &str) -> Option<&(McEliecePublicKey, McElieceSecretKey)> {
        self.mceliece_keys.get(key_id)
    }

    pub fn get_hqc_keys(&self, key_id: &str) -> Option<&(HQCPublicKey, HQCSecretKey)> {
        self.hqc_keys.get(key_id)
    }

    pub fn get_dilithium_keys(&self, key_id: &str) -> Option<&(DilithiumPublicKey, DilithiumSecretKey)> {
        self.dilithium_keys.get(key_id)
    }

    pub fn get_falcon_keys(&self, key_id: &str) -> Option<&(FalconPublicKey, FalconSecretKey)> {
        self.falcon_keys.get(key_id)
    }

    pub fn get_sphincsplus_keys(&self, key_id: &str) -> Option<&(SPHINCSPlusPublicKey, SPHINCSPlusSecretKey)> {
        self.sphincsplus_keys.get(key_id)
    }

    pub fn generate_and_store_keys(&mut self) {
        // Generate Kyber keys
        let (kyber_public_key, kyber_secret_key) = pqcrypto_kyber::keypair();
        self.store_kyber_keys("kyber_key", kyber_public_key, kyber_secret_key);

        // Generate Classic McEliece keys
        let (mceliece_public_key, mceliece_secret_key) = pqcrypto_classicmceliece::keypair();
        self.store_mceliece_keys("mceliece_key", mceliece_public_key, mceliece_secret_key);

        // Generate HQC keys
        let (hqc_public_key, hqc_secret_key) = pqcrypto_hqc::keypair();
        self.store_hqc_keys("hqc_key", hqc_public_key, hqc_secret_key);

        // Generate Dilithium keys
        let (dilithium_public_key, dilithium_secret_key) = pqcrypto_dilithium::keypair();
        self.store_dilithium_keys("dilithium_key", dilithium_public_key, dilithium_secret_key);

        // Generate Falcon keys
        let (falcon_public_key, falcon_secret_key) = pqcrypto_falcon::keypair();
        self.store_falcon_keys("falcon_key", falcon_public_key, falcon_secret_key);

        // Generate SPHINCS+ keys
        let (sphincsplus_public_key, sphincsplus_secret_key) = pqcrypto_sphincsplus::keypair();
        self.store_sphincsplus_keys("sphincsplus_key", sphincsplus_public_key, sphincsplus_secret_key);
    }
}
