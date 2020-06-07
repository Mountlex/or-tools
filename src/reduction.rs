use crate::problem::OptProblemKind;
use crate::algorithm::Algorithm;



pub trait Reduction<V> : OptProblemKind where V : OptProblemKind {

    fn reduce_instance(&self) -> V;
    fn reduce_solution(&self, solution: &V::SolutionKind) -> Self::SolutionKind;

    fn solve_by_reduction(&self, algorithm: &impl Algorithm<V>) -> Self::SolutionKind {
        let reduced_instance = Self::reduce_instance(self);
        let solution = algorithm.run(&reduced_instance);
        self.reduce_solution(&solution)
    }
}



