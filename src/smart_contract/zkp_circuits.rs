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

    pub fn sub_circuit(cs: &mut ConstraintSystemImpl, a: FieldElement, b: FieldElement) -> FieldElement {
        let difference = a.clone() - b.clone();
        let a_var = cs.alloc_variable(a);
        let b_var = cs.alloc_variable(b);
        let difference_var = cs.alloc_variable(difference.clone());

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::sub(Expression::variable(a_var), Expression::variable(b_var)),
            Expression::variable(difference_var),
        ));

        difference
    }

    pub fn div_circuit(cs: &mut ConstraintSystemImpl, a: FieldElement, b: FieldElement) -> FieldElement {
        let quotient = a.clone() / b.clone();
        let a_var = cs.alloc_variable(a);
        let b_var = cs.alloc_variable(b);
        let quotient_var = cs.alloc_variable(quotient.clone());

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::div(Expression::variable(a_var), Expression::variable(b_var)),
            Expression::variable(quotient_var),
        ));

        quotient
    }

    pub fn custom_logic_circuit(cs: &mut ConstraintSystemImpl, a: FieldElement, b: FieldElement, c: FieldElement) -> FieldElement {
        // Custom logic: (a + b) * c
        let sum = a.clone() + b.clone();
        let result = sum.clone() * c.clone();
        let a_var = cs.alloc_variable(a);
        let b_var = cs.alloc_variable(b);
        let c_var = cs.alloc_variable(c);
        let sum_var = cs.alloc_variable(sum);
        let result_var = cs.alloc_variable(result.clone());

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::mul(Expression::add(Expression::variable(a_var), Expression::variable(b_var)), Expression::variable(c_var)),
            Expression::variable(result_var),
        ));

        result
    }
}
