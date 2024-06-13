use crate::zkp::constraint_system::{ConstraintSystem, ConstraintSystemImpl, Expression, PlonkConstraint};
use crate::zkp::math::FieldElement;
use num_bigint::BigUint;

pub struct ZKPCircuits;

impl ZKPCircuits {
    pub fn add_circuit(cs: &mut ConstraintSystemImpl, a: FieldElement, b: FieldElement) -> FieldElement {
        let sum = a.clone() + b.clone();
        let a_var = cs.alloc_variable(a);
        let b_var = cs.alloc_variable(b);
        let sum_var = cs.alloc_variable(sum.clone());

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::add(Expression::variable(a_var), Expression::variable(b_var)),
            Expression::variable(sum_var),
        ));

        sum
    }

    pub fn mul_circuit(cs: &mut ConstraintSystemImpl, a: FieldElement, b: FieldElement) -> FieldElement {
        let product = a.clone() * b.clone();
        let a_var = cs.alloc_variable(a);
        let b_var = cs.alloc_variable(b);
        let product_var = cs.alloc_variable(product.clone());

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::mul(Expression::variable(a_var), Expression::variable(b_var)),
            Expression::variable(product_var),
        ));

        product
    }

    pub fn eq_circuit(cs: &mut ConstraintSystemImpl, a: FieldElement, b: FieldElement) -> FieldElement {
        let eq = if a == b { FieldElement::one(&BigUint::from(1u32) << 256) } else { FieldElement::zero(&BigUint::from(1u32) << 256) };
        let a_var = cs.alloc_variable(a);
        let b_var = cs.alloc_variable(b);
        let eq_var = cs.alloc_variable(eq.clone());

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::variable(a_var),
            Expression::variable(b_var),
        ));

        eq
    }
}
