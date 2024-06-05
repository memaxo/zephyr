use crate::zkp_crate::math::FieldElement;
use std::collections::HashMap;

pub trait ConstraintSystem {
    fn new() -> Self;
    fn alloc_input(&mut self, value: FieldElement) -> Variable;
    fn alloc_variable(&mut self, value: FieldElement) -> Variable;
    fn enforce_constraint(&mut self, lhs: Expression, rhs: Expression);
    fn evaluate(&self, expression: &Expression) -> FieldElement;
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

pub struct RangeConstraint {
    pub variable: Variable,
    pub bitwidth: usize,
}

impl RangeConstraint {
    pub fn new(variable: Variable, bitwidth: usize) -> Self {
        RangeConstraint { variable, bitwidth }
    }

    pub fn enforce(&self, cs: &mut impl ConstraintSystem) {
        let mut accumulator = Expression::constant(FieldElement::zero());
        let one = Expression::constant(FieldElement::one());
        let two = Expression::constant(FieldElement::from(2));

        for i in 0..self.bitwidth {
            let bit = Expression::variable(cs.alloc_variable(FieldElement::zero()));
            cs.enforce_constraint(bit.clone() * (one.clone() - bit.clone()), Expression::constant(FieldElement::zero()));
            accumulator = accumulator + bit * two.pow(FieldElement::from(i as u64));
        }

        cs.enforce_constraint(Expression::variable(self.variable.clone()), accumulator);
    }
}

pub struct EqualsConstraint {
    pub a: Expression,
    pub b: Expression,
}

impl EqualsConstraint {
    pub fn new(a: Expression, b: Expression) -> Self {
        EqualsConstraint { a, b }
    }

    pub fn enforce(&self, cs: &mut impl ConstraintSystem) {
        cs.enforce_constraint(self.a.clone(), self.b.clone());
    }
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

    fn enforce_constraint(&mut self, lhs: Expression, rhs: Expression) {
        self.constraints.push((lhs, rhs));
    }

    fn evaluate(&self, expression: &Expression) -> FieldElement {
        match expression {
            Expression::Constant(value) => value.clone(),
            Expression::Variable(variable) => self.variable_map[variable].clone(),
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
        }
    }
}