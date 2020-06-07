use lp_modeler::dsl::{LpProblem};
use lp_modeler::solvers::{Solution, CbcSolver, GlpkSolver, SolverTrait, Status};
use crate::problem::OptProblemKind;
use crate::algorithm::Algorithm;
use std::collections::HashMap;

impl OptProblemKind for LpProblem {
    type SolutionKind = LpSolution;
    type Cost = f32;
}

pub enum LpSolver {
    CBC,
    GLPK
}

pub enum LpSolution {
    Solved {
        vars: HashMap<String, f32>,
        status: Status,
        value: Option<f32>
    },
    Failed(String)
}

impl From<&Solution<'_>> for LpSolution {
    fn from(solution: &Solution<'_>) -> Self {
                LpSolution::Solved {
                    status: solution.status.clone(),
                    value: solution.eval(),
                    vars: solution.results.clone(),
                }
    }
}

impl Algorithm<LpProblem> for LpSolver {
    fn run(&self, instance: &LpProblem) -> LpSolution {
        let result = match self {
            LpSolver::CBC => {
                CbcSolver::new().run(instance)
            },
            LpSolver::GLPK => {
                GlpkSolver::new().run(instance)
            }
        };
        match result {
            Ok(solution) => LpSolution::from(&solution),
            Err(msg) => LpSolution::Failed(msg)
        }
    }
}



