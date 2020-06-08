use crate::algorithm::Algorithm;

pub trait OptProblemKind: Sized {
    type Solution: SolutionKind<Self>;
    type Cost;

    fn run<T: Algorithm<Self>>(&self, algorithm: T) -> Self::Solution {
        algorithm.run(self)
    }
}

pub trait SolutionKind<P>
where
    P: OptProblemKind,
{
    fn cost(&self, instance: &P) -> Option<<P as OptProblemKind>::Cost>;
}
