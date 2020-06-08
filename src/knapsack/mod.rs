use crate::primitives::NumericKind;
use crate::problem::OptProblemKind;
use std::fmt::Display;

pub mod algorithms;
pub mod reductions;

#[derive(Clone, Debug)]
pub struct Instance<I>
where
    I: Item,
{
    items: Vec<I>,
    size: f64,
}

impl<I> Instance<I>
where
    I: Item,
{
    pub fn new(items: Vec<I>, size: f64) -> Self {
        if items.iter().any(|item| item.weight() <= 0.0) {
            panic!("Item weights must be positive!");
        }
        Instance {
            items: items,
            size: size,
        }
    }

    pub fn items(&self) -> &[I] {
        &self.items
    }

    pub fn number_of_items(&self) -> usize {
        self.items.len()
    }

    pub fn bag_size(&self) -> &f64 {
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

impl<I> OptProblemKind for Instance<I>
where
    I: Item,
{
    type SolutionKind = Solution;
    type Cost = f64;
}

#[derive(Clone, Debug)]
pub enum Solution {
    Solved { packed_items: Vec<usize> },
    Infeasible,
    Failed(String),
}

impl Solution {
    pub fn is_solved(&self) -> bool {
        match self {
            Solution::Solved { .. } => true,
            _ => false,
        }
    }

    pub fn as_solution(self) -> Option<Vec<usize>> {
        match self {
            Solution::Solved { packed_items } => Some(packed_items),
            _ => None,
        }
    }
}

pub trait Item: Display + Clone {
    fn weight(&self) -> f64;
    fn cost(&self) -> f64;
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultItem<T>
where
    T: NumericKind,
{
    cost: T,
    weight: T,
}

impl<T> Display for DefaultItem<T>
where
    T: NumericKind,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item[c = {}, w = {}]", self.cost, self.weight)
    }
}

impl<T> Item for DefaultItem<T>
where
    T: NumericKind,
{
    fn weight(&self) -> f64 {
        self.weight.into()
    }
    fn cost(&self) -> f64 {
        self.cost.into()
    }
}

impl<T> From<(Vec<(T, T)>, T)> for Instance<DefaultItem<T>>
where
    T: NumericKind,
{
    fn from(input: (Vec<(T, T)>, T)) -> Self {
        let items: Vec<DefaultItem<T>> = input
            .0
            .into_iter()
            .map(|(cost, weight)| DefaultItem::from((cost, weight)))
            .collect();
        Instance::new(items, input.1.into())
    }
}

impl<T> From<(T, T)> for DefaultItem<T>
where
    T: NumericKind,
{
    fn from(input: (T, T)) -> Self {
        DefaultItem {
            cost: input.0,
            weight: input.1,
        }
    }
}

// Reductions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_instance_from_works() {
        let instance = Instance::from((vec![(1, 2), (2, 3), (3, 4)], 5));
        assert_eq!(3, instance.number_of_items());
        assert_eq!(5.0, *instance.bag_size());
    }
}
