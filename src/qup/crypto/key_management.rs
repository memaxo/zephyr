use rand::Rng;

pub trait KeyPairGenerator {
    type PublicKey;
    type SecretKey;

    fn generate(rng: &mut impl Rng) -> Self;
}
