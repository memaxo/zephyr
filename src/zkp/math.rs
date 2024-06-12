use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use c_kzg::{Blob, Bytes32, Bytes48, KzgCommitment, KzgProof, KzgSettings, CkzgError, BYTES_PER_BLOB, BYTES_PER_COMMITMENT, BYTES_PER_FIELD_ELEMENT, BYTES_PER_G1_POINT, BYTES_PER_G2_POINT, BYTES_PER_PROOF, FIELD_ELEMENTS_PER_BLOB};
use sha2::{Sha256, Digest};
type Hasher = Sha256;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FieldElement(BigUint);

impl FieldElement {
    pub fn new(value: BigUint, modulus: &BigUint) -> Self {
        FieldElement(value % modulus)
    }

    pub fn zero(modulus: &BigUint) -> Self {
        FieldElement(BigUint::zero() % modulus)
    }

    pub fn one(modulus: &BigUint) -> Self {
        FieldElement(BigUint::one() % modulus)
    }

    pub fn random(modulus: &BigUint) -> Self {
        let mut rng = rand::thread_rng();
        let value = rng.gen_biguint(modulus.bits());
        FieldElement(value % modulus)
    }

    pub fn pow(&self, exponent: &BigUint, modulus: &BigUint) -> Self {
        let mut result = FieldElement::one(modulus);
        let mut base = self.clone();
        let mut exp = exponent.clone();

        while !exp.is_zero() {
            if exp.bit(0) {
                result = result * base.clone();
            }
            base = base.clone() * base;
            exp >>= 1;
        }

        result
    }

    pub fn generator(modulus: &BigUint) -> Self {
        // TODO: Implement a proper generator selection algorithm
        FieldElement::new(BigUint::from(2u32), modulus)
    }

    pub fn get_domain(size: usize, modulus: &BigUint) -> Vec<FieldElement> {
        let mut domain = Vec::with_capacity(size);
        let generator = FieldElement::generator(modulus);

        for i in 0..size {
            let element = generator.pow(&BigUint::from(i as u32), modulus);
            domain.push(element);
        }

        domain
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.0 + other.0;
        if result >= self.modulus() {
            result -= self.modulus();
        }
        FieldElement(result)
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self.0 - other.0;
        if result >= self.modulus() {
            result += self.modulus();
        }
        FieldElement(result)
    }
}

impl SubAssign for FieldElement {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        FieldElement(self.0 * other.0 % self.modulus())
    }
}

impl MulAssign for FieldElement {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

#[derive(Clone, Debug)]
pub struct PolynomialCommitment {
    pub coefficients: Vec<FieldElement>,
    pub commitment: KzgCommitment,
}

impl PolynomialCommitment {
    pub fn new(coefficients: Vec<FieldElement>, settings: &KzgSettings) -> Result<Self, CkzgError> {
        let blob = Blob::from(coefficients.clone());
        let commitment = KzgCommitment::new(&blob, settings)?;
        Ok(PolynomialCommitment { coefficients, commitment })
    }

    pub fn evaluate(&self, point: &FieldElement) -> FieldElement {
        let mut result = FieldElement::zero(&self.modulus());
        let mut power = FieldElement::one(&self.modulus());

        for coefficient in &self.coefficients {
            result += coefficient.clone() * power.clone();
            power *= point.clone();
        }

        result
    }

    pub fn verify(&self, point: &FieldElement, value: &FieldElement, proof: &KzgProof, settings: &KzgSettings) -> Result<bool, CkzgError> {
        let commitment = self.commitment.clone();
        let result = KzgCommitment::verify(&commitment, &point.to_bytes(), &value.to_bytes(), &proof, settings)?;
        Ok(result)
    }
}
