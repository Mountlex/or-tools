use crate::knapsack::{DefaultItem, Instance, Item, Solution};
use crate::program::{LpSolution, MathProgram};
use crate::reduction::Reduction;

use lp_modeler::dsl::*;

impl Reduction<MathProgram> for Instance<DefaultItem<f32>> {
    fn reduce_instance(&self) -> MathProgram {
        let mut model = LpProblem::new("knapsack", LpObjective::Maximize);
        let vars: Vec<(&DefaultItem<f32>, LpBinary)> = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| (item, LpBinary::new(&format!("x_{}", index))))
            .collect();

        let obj_vec: Vec<LpExpression> =
            vars.iter().map(|(item, var)| *item.cost() * var).collect();
        model += obj_vec.sum();

        model += sum(&vars, |(item, var)| *item.weight() * var).le(self.size);

        model.into()
    }

    fn reduce_solution(&self, solution: &LpSolution) -> Solution {
        match solution {
            LpSolution::Failed(msg) => Solution::Failed(String::from(msg)),
            LpSolution::Infeasible => Solution::Infeasible,
            LpSolution::Unbounded => {
                panic!("LP solution unbounded for knapsack instance. This should not happen!")
            }
            LpSolution::Solved { vars, .. } => {
                let packed = self
                    .items
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *vars.get(&format!("x_{}", index)).unwrap() == 1.0)
                    .map(|(index, _)| index)
                    .collect();
                Solution::Solved {
                    packed_items: packed,
                }
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use crate::program::LpSolver;

    #[test]
    fn reduction_works() {
        let instance = &Instance::from((vec![(1.0, 2.0), (2.0, 3.0), (2.0, 4.0)], 5.0));
        let program = instance.reduce_instance();
        assert_eq!(3, program.number_of_variables());
        assert_eq!(1, program.number_of_constraints());
    }

    #[test]
    fn solving_by_reduction_works() {
        let instance = &Instance::from((vec![(1.0, 2.0), (2.0, 3.0), (2.0, 4.0)], 5.0));
        let solution = instance.solve_by_reduction(&LpSolver::CBC);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![0, 1])
    }
}
