use crate::problem::OptProblemKind;
use crate::algorithm::Algorithm;

pub trait Reduction<T,V> where T : OptProblemKind, V : OptProblemKind {

    fn reduce_instance(instance: &T) -> V;
    fn reduce_solution(solution: &V::SolutionKind) -> T::SolutionKind;

    fn solve_by_reduction(&self, instance: &T, algorithm: &impl Algorithm<V>) -> T::SolutionKind {
        let reduced_instance = Self::reduce_instance(instance);
        let solution = algorithm.run(&reduced_instance);
        Self::reduce_solution(&solution)
    }
}