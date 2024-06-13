use crate::zkp::constraint_system::{ConstraintSystem, ConstraintSystemImpl, Expression, PlonkConstraint};
use crate::zkp::math::FieldElement;
use num_bigint::BigUint;

pub struct StateTransitionCircuits;

impl StateTransitionCircuits {
    pub fn verify_account_balance(cs: &mut ConstraintSystemImpl, balance: FieldElement, expected_balance: FieldElement) -> FieldElement {
        let balance_var = cs.alloc_variable(balance);
        let expected_balance_var = cs.alloc_variable(expected_balance);

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::variable(balance_var),
            Expression::variable(expected_balance_var),
        ));

        expected_balance
    }

    pub fn verify_state_transition(cs: &mut ConstraintSystemImpl, old_state: FieldElement, new_state: FieldElement, transition: FieldElement) -> FieldElement {
        let old_state_var = cs.alloc_variable(old_state);
        let new_state_var = cs.alloc_variable(new_state);
        let transition_var = cs.alloc_variable(transition);

        let computed_new_state = old_state.clone() + transition.clone();
        let computed_new_state_var = cs.alloc_variable(computed_new_state.clone());

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::add(Expression::variable(old_state_var), Expression::variable(transition_var)),
            Expression::variable(computed_new_state_var),
        ));

        cs.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::variable(computed_new_state_var),
            Expression::variable(new_state_var),
        ));

        new_state
    }
}
