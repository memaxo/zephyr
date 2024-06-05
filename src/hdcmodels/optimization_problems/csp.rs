use std::collections::HashMap;

pub trait Variable {
    fn get_domain(&self) -> &[usize];
}

pub trait Constraint<V: Variable> {
    fn is_satisfied(&self, assignment: &HashMap<usize, usize>, variables: &[V]) -> bool;
}

pub struct CSP<V: Variable, C: Constraint<V>> {
    pub variables: Vec<V>,
    pub constraints: Vec<C>,
}

impl<V: Variable, C: Constraint<V>> CSP<V, C> {
    pub fn new(variables: Vec<V>, constraints: Vec<C>) -> Self {
        CSP {
            variables,
            constraints,
        }
    }

    pub fn solve(&self) -> Option<HashMap<usize, usize>> {
        let mut assignment = HashMap::new();
        self.backtrack(&mut assignment, 0)
    }

    fn backtrack(
        &self,
        assignment: &mut HashMap<usize, usize>,
        index: usize,
    ) -> Option<HashMap<usize, usize>> {
        if index == self.variables.len() {
            return Some(assignment.clone());
        }

        for &value in self.variables[index].get_domain() {
            assignment.insert(index, value);

            if self
                .constraints
                .iter()
                .all(|constraint| constraint.is_satisfied(assignment, &self.variables))
            {
                if let Some(solution) = self.backtrack(assignment, index + 1) {
                    return Some(solution);
                }
            }

            assignment.remove(&index);
        }

        None
    }
}
