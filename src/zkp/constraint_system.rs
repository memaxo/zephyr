use crate::zkp_crate::math::FieldElement;
use std::collections::HashMap;
use thiserror::Error;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};

pub trait ConstraintSystem {
    fn new() -> Self;
    fn alloc_input(&mut self, value: FieldElement) -> Variable;
    fn alloc_variable(&mut self, value: FieldElement) -> Variable;
    fn enforce_plonk_constraint(&mut self, constraint: PlonkConstraint);
    fn evaluate(&self, expression: &Expression) -> FieldElement;
pub struct ConstraintSystemImpl {
    pub inputs: Vec<FieldElement>,
    pub variables: Vec<FieldElement>,
    pub plonk_constraints: Vec<PlonkConstraint>,
    pub variable_map: HashMap<Variable, FieldElement>,
    pub custom_gates: Vec<CustomGate>,
}

impl ConstraintSystem for ConstraintSystemImpl {
    fn new() -> Self {
        ConstraintSystemImpl {
            inputs: Vec::new(),
            variables: Vec::new(),
            plonk_constraints: Vec::new(),
            variable_map: HashMap::new(),
            custom_gates: Vec::new(),
        }
    }

    fn alloc_input(&mut self, value: FieldElement) -> Variable {
        let index = self.inputs.len();
        self.inputs.push(value);
        Variable(index)
    }

    fn alloc_variable(&mut self, value: FieldElement) -> Variable {
        let index = self.variables.len();
        self.variables.push(value);
        Variable(index)
    }

    fn enforce_plonk_constraint(&mut self, constraint: PlonkConstraint) {
        self.plonk_constraints.push(constraint);
    }

    fn evaluate(&self, expression: &Expression) -> FieldElement {
        let result = match expression {
            Expression::Constant(value) => value.clone(),
            Expression::Variable(variable) => self.variable_map.get(variable)
                .ok_or_else(|| format!("Variable {:?} not found", variable))?
                .clone(),
            Expression::Add(lhs, rhs) => {
                let lhs_value = self.evaluate(lhs);
                let rhs_value = self.evaluate(rhs);
                lhs_value + rhs_value
            }
            Expression::Mul(lhs, rhs) => {
                let lhs_value = self.evaluate(lhs);
                let rhs_value = self.evaluate(rhs);
                lhs_value * rhs_value
            }
        };
        result
    }

    pub fn add_custom_gate(&mut self, gate: CustomGate) {
        self.custom_gates.push(gate);
    }

    pub fn enforce_custom_gates(&mut self) {
        for gate in &self.custom_gates {
            gate.enforce(self);
        }
    }

    fn enforce_range_proof(&mut self, value_var: Variable, range_proof: RangeProof, pc_gens: &PedersenGens, bp_gens: &BulletproofGens) {
        // 1. Extract commitments and proof data from the range_proof
        let (commit, proof) = range_proof.to_bytes();
        let value_commit = self.alloc_variable(FieldElement::from_bytes(&commit));

        // 2. Add constraints to verify the inner product argument
        let (ipa_vars, ipa_constraints) = self.verify_inner_product_argument(&proof, bp_gens);
        for var in ipa_vars {
            self.alloc_variable(var);
        }
        for constraint in ipa_constraints {
            self.enforce_plonk_constraint(constraint);
        }

        // 3. Add constraints to verify the Pedersen commitments  
        let (com_vars, com_constraints) = self.verify_pedersen_commitments(&proof, pc_gens);
        for var in com_vars {
            self.alloc_variable(var);
        }
        for constraint in com_constraints {
            self.enforce_plonk_constraint(constraint);
        }

        // 4. Add constraints to check the range proof's validity
        self.enforce_plonk_constraint(PlonkConstraint::new(
            Expression::variable(value_var),
            Expression::variable(value_commit),
        ));
    }

    fn verify_inner_product_argument(&self, proof: &[u8], bp_gens: &BulletproofGens) -> (Vec<FieldElement>, Vec<PlonkConstraint>) {
        // Implement verification of inner product argument here
        // Return allocated variables and constraints
        todo!()
    }

    fn verify_pedersen_commitments(&self, proof: &[u8], pc_gens: &PedersenGens) -> (Vec<FieldElement>, Vec<PlonkConstraint>) {
        // Implement verification of Pedersen commitments here 
        // Return allocated variables and constraints
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable(usize);

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(FieldElement),
    Variable(Variable),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn constant(value: FieldElement) -> Self {
        Expression::Constant(value)
    }

    pub fn variable(variable: Variable) -> Self {
        Expression::Variable(variable)
    }

    pub fn add(lhs: Expression, rhs: Expression) -> Self {
        Expression::Add(Box::new(lhs), Box::new(rhs))
    }

    pub fn mul(lhs: Expression, rhs: Expression) -> Self {
        Expression::Mul(Box::new(lhs), Box::new(rhs))
    }
}

#[derive(Clone, Debug)]
pub struct PlonkConstraint {
    pub lhs: Expression,
    pub rhs: Expression,
}

impl PlonkConstraint {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        PlonkConstraint { lhs, rhs }
    }
}

pub enum CustomGate {
    RangeProof { variable: Variable, bitwidth: usize },
    ArithmeticCircuit { a: Expression, b: Expression },
}

impl CustomGate {
    pub fn enforce(&self, cs: &mut impl ConstraintSystem) {
        match self {
            CustomGate::RangeProof { variable, bitwidth } => {
                let mut accumulator = Expression::constant(FieldElement::zero());
                let one = Expression::constant(FieldElement::one());
                let two = Expression::constant(FieldElement::from(2));

                for i in 0..*bitwidth {
                    let bit = Expression::variable(cs.alloc_variable(FieldElement::zero()));
                    cs.enforce_plonk_constraint(PlonkConstraint::new(bit.clone() * (one.clone() - bit.clone()), Expression::constant(FieldElement::zero())));
                    accumulator = accumulator + bit * two.pow(FieldElement::from(i as u64));
                }

                cs.enforce_plonk_constraint(PlonkConstraint::new(Expression::variable(variable.clone()), accumulator));
            }
            CustomGate::ArithmeticCircuit { a, b } => {
                cs.enforce_plonk_constraint(PlonkConstraint::new(a.clone(), b.clone()));
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ConstraintSystemError {
    #[error("Constraint failed: {0}")]
    ConstraintFailed(String),
}

pub struct ConstraintSystemImpl {
    pub inputs: Vec<FieldElement>,
    pub variables: Vec<FieldElement>,
    pub constraints: Vec<(Expression, Expression)>,
    pub variable_map: HashMap<Variable, FieldElement>,
}

impl ConstraintSystem for ConstraintSystemImpl {
    fn new() -> Self {
        ConstraintSystemImpl {
            inputs: Vec::new(),
            variables: Vec::new(),
            constraints: Vec::new(),
            variable_map: HashMap::new(),
        }
    }

    fn alloc_input(&mut self, value: FieldElement) -> Variable {
        let index = self.inputs.len();
        self.inputs.push(value);
        Variable(index)
    }

    fn alloc_variable(&mut self, value: FieldElement) -> Variable {
        let index = self.variables.len();
        self.variables.push(value);
        Variable(index)
    }

    fn enforce_constraint(&mut self, lhs: Expression, rhs: Expression) -> Result<(), ConstraintSystemError> {
        self.constraints.push((lhs, rhs));
        Ok(())
    }

    fn evaluate(&self, expression: &Expression) -> Result<FieldElement, ConstraintSystemError> {
        let result = match expression {
            Expression::Constant(value) => value.clone(),
            Expression::Variable(variable) => self.variable_map.get(variable)
                .ok_or_else(|| ConstraintSystemError::ConstraintFailed(format!("Variable {:?} not found", variable)))?
                .clone(),
            Expression::Add(lhs, rhs) => {
                let lhs_value = self.evaluate(lhs)?;
                let rhs_value = self.evaluate(rhs)?;
                lhs_value + rhs_value
            }
            Expression::Mul(lhs, rhs) => {
                let lhs_value = self.evaluate(lhs)?;
                let rhs_value = self.evaluate(rhs)?;
                lhs_value * rhs_value
            }
        };
        Ok(result)
        }
    }
}
