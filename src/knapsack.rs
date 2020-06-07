use crate::problem::OptProblemKind;
use crate::program::{LpSolution, LpSolver, MathProgram};
use crate::reduction::Reduction;
use lp_modeler::dsl::*;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;

#[derive(Clone, Debug)]
struct Instance<I>
where
    I: Item,
{
    items: Vec<I>,
    size: I::Weight,
}

impl<I> Instance<I>
where
    I: Item,
{
    pub fn number_of_items(&self) -> usize {
        self.items.len()
    }

    pub fn bag_size(&self) -> &I::Weight {
        &self.size
    }
}

impl<I> Display for Instance<I>
where
    I: Item,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Knapsack instance\n   - size = {}\n   - items = {{ {} }}",
            self.size,
            self.items
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(&", ")
        )
    }
}

impl<'a, I> OptProblemKind for &'a Instance<I>
where
    I: Item,
{
    type SolutionKind = Solution<&'a I>;
    type Cost = I::Cost;
}

#[derive(Clone, Debug)]
pub enum Solution<I> {
    Solved { packed_items: Vec<I> },
    Infeasible,
    Failed(String),
}

impl<'a, I> Solution<&'a I>
where
    I: Item,
{
    pub fn is_solved(&self) -> bool {
        match self {
            Solution::Solved { .. } => true,
            _ => false,
        }
    }

    pub fn as_solution(self) -> Option<Vec<&'a I>> {
        match self {
            Solution::Solved { packed_items } => Some(packed_items),
            _ => None,
        }
    }
}

pub trait Item: Display {
    type Weight: Add + Sum + PartialEq + Display;
    type Cost: Add + Sum + PartialEq + Display;

    fn weight(&self) -> &Self::Weight;
    fn cost(&self) -> &Self::Cost;
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultItem<T>
where
    T: Add + Sum + PartialEq + Display,
{
    cost: T,
    weight: T,
}

impl<T> Display for DefaultItem<T>
where
    T: Add + Sum + PartialEq + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item[c = {}, w = {}]", self.cost, self.weight)
    }
}

impl<T> Item for DefaultItem<T>
where
    T: Add + Sum + PartialEq + Display,
{
    type Weight = T;
    type Cost = T;

    fn weight(&self) -> &Self::Weight {
        &self.weight
    }
    fn cost(&self) -> &Self::Cost {
        &self.cost
    }
}

impl<T> From<(Vec<(T, T)>, T)> for Instance<DefaultItem<T>>
where
    T: Add + Sum + PartialEq + Display,
{
    fn from(input: (Vec<(T, T)>, T)) -> Self {
        let items: Vec<DefaultItem<T>> = input
            .0
            .into_iter()
            .map(|(cost, weight)| DefaultItem::from((cost, weight)))
            .collect();
        Instance {
            items: items,
            size: input.1,
        }
    }
}

impl<T> From<(T, T)> for DefaultItem<T>
where
    T: Add + Sum + PartialEq + Display,
{
    fn from(input: (T, T)) -> Self {
        DefaultItem {
            cost: input.0,
            weight: input.1,
        }
    }
}

// Reductions

impl<'a> Reduction<MathProgram> for &'a Instance<DefaultItem<f32>> {
    fn reduce_instance(&self) -> MathProgram {
        let mut model = LpProblem::new("knapsack", LpObjective::Maximize);
        let vars: Vec<(&DefaultItem<f32>, LpBinary)> = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| (item, LpBinary::new(&format!("x_{}", index))))
            .collect();

        let obj_vec: Vec<LpExpression> =
            vars.iter().map(|(item, var)| *item.cost() * var).collect();
        model += obj_vec.sum();

        model += sum(&vars, |(item, var)| *item.weight() * var).le(self.size);

        model.into()
    }

    fn reduce_solution(&self, solution: &LpSolution) -> Solution<&'a DefaultItem<f32>> {
        match solution {
            LpSolution::Failed(msg) => Solution::Failed(String::from(msg)),
            LpSolution::Infeasible => Solution::Infeasible,
            LpSolution::Unbounded => {
                panic!("LP solution unbounded for knapsack instance. This should not happen!")
            }
            LpSolution::Solved { vars, .. } => {
                let packed = self
                    .items
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *vars.get(&format!("x_{}", index)).unwrap() == 1.0)
                    .map(|(_, item)| item)
                    .collect();
                Solution::Solved {
                    packed_items: packed,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_instance_from_works() {
        let instance = Instance::from((vec![(1, 2), (2, 3), (3, 4)], 5));
        assert_eq!(3, instance.number_of_items());
        assert_eq!(5, *instance.bag_size());
    }

    #[test]
    fn reduction_works() {
        let instance = &Instance::from((vec![(1.0, 2.0), (2.0, 3.0), (2.0, 4.0)], 5.0));
        let program = instance.reduce_instance();
        assert_eq!(3, program.number_of_variables());
        assert_eq!(1, program.number_of_constraints());
    }

    #[test]
    fn solving_by_reduction_works() {
        let instance = &Instance::from((vec![(1.0, 2.0), (2.0, 3.0), (2.0, 4.0)], 5.0));
        println!("{}", instance);
        let solution = instance.solve_by_reduction(&LpSolver::CBC);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(
            items,
            vec![
                &DefaultItem::from((1.0, 2.0)),
                &DefaultItem::from((2.0, 3.0))
            ]
        )
    }
}
