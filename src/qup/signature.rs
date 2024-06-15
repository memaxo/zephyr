use crate::qup::signature::verify_signature;

pub fn verify_signature(data: &[u8], signature: &[u8], public_key: &DilithiumPublicKey) -> bool {
    verify(data, signature, public_key).is_ok()
}
