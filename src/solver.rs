use crate::problem::OptProblemKind;

pub trait Solver<V> where V : OptProblemKind {

    fn solve(&self, instance: V) -> V::Solution;
}
