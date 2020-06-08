use crate::algorithm::Algorithm;
use crate::problem::{OptProblemKind, SolutionKind};
use lp_modeler::dsl::LpProblem;
use lp_modeler::solvers::{CbcSolver, GlpkSolver, Solution, SolverTrait, Status};
use std::collections::HashMap;

pub struct MathProgram(LpProblem);

impl MathProgram {
    pub fn number_of_variables(&self) -> usize {
        self.0.variables().len()
    }
    pub fn number_of_constraints(&self) -> usize {
        self.0.constraints.len()
    }
}

impl OptProblemKind for MathProgram {
    type Solution = LpSolution;
    type Cost = f32;
}

pub enum LpSolver {
    CBC,
    GLPK,
}

pub enum SolutionType {
    Optimal,
    SubOptimal,
}

pub enum LpSolution {
    Solved {
        vars: HashMap<String, f32>,
        kind: SolutionType,
        value: Option<f32>,
    },
    Infeasible,
    Unbounded,
    Failed(String),
}

impl SolutionKind<MathProgram> for LpSolution {
    fn cost(&self, instance: &MathProgram) -> Option<f32> {
        match self {
            LpSolution::Solved { value, .. } => *value,
            _ => None,
        }
    }
}

impl From<&Solution<'_>> for LpSolution {
    fn from(solution: &Solution<'_>) -> Self {
        match solution.status {
            Status::Optimal => LpSolution::Solved {
                kind: SolutionType::Optimal,
                value: solution.eval(),
                vars: solution.results.clone(),
            },
            Status::SubOptimal => LpSolution::Solved {
                kind: SolutionType::SubOptimal,
                value: solution.eval(),
                vars: solution.results.clone(),
            },
            Status::Infeasible => LpSolution::Infeasible,
            Status::Unbounded => LpSolution::Unbounded,
            Status::NotSolved => LpSolution::Failed(String::from("Not solved")),
        }
    }
}

impl From<LpProblem> for MathProgram {
    fn from(instance: LpProblem) -> MathProgram {
        MathProgram(instance)
    }
}

impl Algorithm<MathProgram> for LpSolver {
    fn run(&self, instance: &MathProgram) -> LpSolution {
        let result = match self {
            LpSolver::CBC => CbcSolver::new().run(&instance.0),
            LpSolver::GLPK => GlpkSolver::new().run(&instance.0),
        };
        match result {
            Ok(solution) => LpSolution::from(&solution),
            Err(msg) => LpSolution::Failed(msg),
        }
    }
}
