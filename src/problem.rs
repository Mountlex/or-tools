use crate::algorithm::Algorithm;

pub trait OptProblemKind: Sized {
    type SolutionKind;
    type Cost;

    fn run<T : Algorithm<Self>>(&self, algorithm: T) -> Self::SolutionKind {
        algorithm.run(self)
    }
}



