
pub trait OptProblemKind {
    type Solution;
    type Cost;

    fn cost(solution: &Self::Solution) -> Self::Cost;
}

