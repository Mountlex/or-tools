use crate::problem::OptProblemKind;

pub trait Algorithm<V> where V : OptProblemKind {
    fn run(&self, instance: &V) -> V::SolutionKind;
}
