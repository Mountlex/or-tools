use lp_modeler::dsl::{LpProblem};
use lp_modeler::solvers::{Solution, CbcSolver, GlpkSolver, SolverTrait, Status};
use crate::problem::OptProblemKind;
use crate::algorithm::Algorithm;
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
    type SolutionKind = LpSolution;
    type Cost = f32;
}

pub enum LpSolver {
    CBC,
    GLPK
}

pub enum SolutionKind {
    Optimal,
    SubOptimal
}

pub enum LpSolution {
    Solved {
        vars: HashMap<String, f32>,
        kind: SolutionKind,
        value: Option<f32>
    },
    Infeasible,
    Unbounded,
    Failed(String)
}

impl From<&Solution<'_>> for LpSolution {
    fn from(solution: &Solution<'_>) -> Self {
        match solution.status {
            Status::Optimal => {
                LpSolution::Solved {
                    kind: SolutionKind::Optimal,
                    value: solution.eval(),
                    vars: solution.results.clone(),
                }
            },
            Status::SubOptimal => {
                LpSolution::Solved {
                    kind: SolutionKind::SubOptimal,
                    value: solution.eval(),
                    vars: solution.results.clone(),
                }
            },
            Status::Infeasible => LpSolution::Infeasible,
            Status::Unbounded => LpSolution::Unbounded,
            Status::NotSolved => LpSolution::Failed(String::from("Not solved"))
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
            LpSolver::CBC => {
                CbcSolver::new().run(&instance.0)
            },
            LpSolver::GLPK => {
                GlpkSolver::new().run(&instance.0)
            }
        };
        match result {
            Ok(solution) => LpSolution::from(&solution),
            Err(msg) => LpSolution::Failed(msg)
        }
    }
}



