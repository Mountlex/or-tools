use crate::problem::OptProblemKind;
use crate::solver::Solver;

pub trait Reduction<T,V> where T : OptProblemKind, V : OptProblemKind {

    fn reduce_instance(instance: &T) -> V;
    fn reduce_solution(solution: &V::Solution) -> T::Solution;

    fn solve_by_reduction(&self, instance: &T, solver: &impl Solver<V>) -> T::Solution {
        let reduced_instance = Self::reduce_instance(instance);
        let solution = solver.solve(reduced_instance);
        Self::reduce_solution(&solution)
    }
}