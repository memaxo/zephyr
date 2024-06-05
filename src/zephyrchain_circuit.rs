use winterfell::{
    Air,
    AirContext,
    Assertion,
    EvaluationFrame,
    FieldElement,
    ProofOptions,
    StarkError,
    TraceGenerator,
    TraceTable,
    TransitionConstraintDegree,
};
use winterfell::math::fields::f128::BaseElement;

const NUM_COLUMNS: usize = 5;
const NUM_CONSTRAINTS: usize = 4;

pub struct ZephyrChainCircuit {
    pub context: AirContext<BaseElement>,
}

impl Air for ZephyrChainCircuit {
    type BaseField = BaseElement;
    type PublicInputs = Vec<BaseElement>;

    fn new(trace_length: usize, public_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        let degrees = vec![TransitionConstraintDegree::new(2); NUM_CONSTRAINTS];
        let context = AirContext::new(trace_length, NUM_COLUMNS, degrees, options);
        Self { context }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition_constraints<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _public_inputs: &Self::PublicInputs,
        constraints: &mut [E],
    ) -> Result<(), StarkError> {
        let current = frame.current();
        let next = frame.next();

        constraints[0] = current[3] - next[3];
        constraints[1] = current[4] - current[2] * current[3];
        constraints[2] = current[2] - next[2];

        Ok(())
    }

    fn get_assertions(&self) -> Result<Vec<Assertion<Self::BaseField>>, StarkError> {
        let mut assertions = Vec::new();

        assertions.push(Assertion::single(
            3,
            self.context.trace_length() - 1,
            BaseElement::new(1),
        ));

        Ok(assertions)
    }
}

impl TraceGenerator for ZephyrChainCircuit {
    fn generate(&self) -> Result<TraceTable<BaseElement>, StarkError> {
        Ok(TraceTable::new(NUM_COLUMNS, self.context.trace_length()))
    }
}