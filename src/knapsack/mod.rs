use crate::primitives::Numeric;
use crate::problem::OptProblemKind;
use std::fmt::Display;
use std::marker::PhantomData;

pub mod algorithms;
pub mod reductions;

#[derive(Clone, Debug)]
pub struct Instance<I, C, W>
where
    I: Item<C, W>,
    C: Numeric,
    W: Numeric,
{
    items: Vec<I>,
    size: W,
    cost_type: PhantomData<C>,
}

impl<I, C, W> Instance<I, C, W>
where
    I: Item<C, W>,
    C: Numeric,
    W: Numeric,
{
    pub fn new(items: Vec<I>, size: W) -> Self {
        //if items.iter().any(|item| item.weight() <= 0.0) {
        //    panic!("Item weights must be positive!");
        //}
        Instance {
            items: items,
            size: size,
            cost_type: PhantomData,
        }
    }

    pub fn items(&self) -> &[I] {
        &self.items
    }

    pub fn number_of_items(&self) -> usize {
        self.items.len()
    }

    pub fn bag_size(&self) -> &W {
        &self.size
    }
}

impl<I, C, W> Display for Instance<I, C, W>
where
    I: Item<C, W>,
    C: Numeric,
    W: Numeric,
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

impl<I, C, W> OptProblemKind for Instance<I, C, W>
where
    I: Item<C, W>,
    C: Numeric,
    W: Numeric,
{
    type SolutionKind = Solution;
    type Cost = C;
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

pub trait Item<C, W>: Display + Clone
where
    C: Numeric,
    W: Numeric,
{
    fn weight(&self) -> &W;
    fn cost(&self) -> &C;
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultItem<T>
where
    T: Numeric,
{
    cost: T,
    weight: T,
}

impl<T> Display for DefaultItem<T>
where
    T: Numeric,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item[c = {}, w = {}]", self.cost, self.weight)
    }
}

impl<T> Item<T, T> for DefaultItem<T>
where
    T: Numeric,
{
    fn weight(&self) -> &T {
        &self.weight
    }
    fn cost(&self) -> &T {
        &self.cost
    }
}

impl<T> From<(Vec<(T, T)>, T)> for Instance<DefaultItem<T>, T, T>
where
    T: Numeric,
{
    fn from(input: (Vec<(T, T)>, T)) -> Self {
        let items: Vec<DefaultItem<T>> = input
            .0
            .into_iter()
            .map(|(cost, weight)| DefaultItem::from((cost, weight)))
            .collect();
        Instance::new(items, input.1)
    }
}

impl<T> From<(T, T)> for DefaultItem<T>
where
    T: Numeric,
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
        assert_eq!(5, *instance.bag_size());
    }
}
