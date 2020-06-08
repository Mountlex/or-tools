use crate::algorithm::Algorithm;
use crate::knapsack::{Instance, Item, Solution};
use crate::primitives::Numeric;
use std::collections::HashMap;

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

pub struct Greedy;

impl<I> Algorithm<Instance<I, f64, f64>> for Greedy
where
    I: Item<f64, f64>,
{
    fn run(&self, instance: &Instance<I, f64, f64>) -> Solution {
        let mut indexed_items: Vec<(usize, &I)> = instance.items().iter().enumerate().collect();
        indexed_items.sort_by(|(_, b), (_, a)| {
            (a.cost() / a.weight())
                .partial_cmp(&(b.cost() / b.weight()))
                .unwrap()
        });
        let mut weight: f64 = 0.0;
        let mut packed: Vec<usize> = Vec::new();
        for (index, item) in indexed_items {
            weight += item.weight();
            if weight > (*instance.bag_size()) {
                break;
            } else {
                packed.push(index);
            }
        }
        packed.sort();
        Solution::Solved {
            packed_items: packed,
        }
    }
}

pub struct DynamicProgram;

impl<I, C> Algorithm<Instance<I, C, usize>> for DynamicProgram
where
    I: Item<C, usize>,
    C: Numeric,
{
    fn run(&self, instance: &Instance<I, C, usize>) -> Solution {
        let mut values = vec![
            vec![C::zero(); *instance.bag_size() + 1 as usize];
            instance.number_of_items() + 1
        ];

        for (i, item) in instance.items().iter().enumerate() {
            for j in 1..*instance.bag_size() + 1 {
                values[i + 1][j] = if *item.weight() > j {
                    values[i][j]
                } else {
                    max!(values[i][j], values[i][j - item.weight()] + *item.cost())
                }
            }
        }

        let mut packed: Vec<usize> = Vec::new();
        let mut left_weight = *instance.bag_size();

        for (i, item) in instance.items().iter().enumerate().rev() {
            if values[i + 1][left_weight] != values[i][left_weight] {
                packed.push(i);
                left_weight -= item.weight();
            }
        }
        packed.sort();
        Solution::Solved {
            packed_items: packed,
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use crate::problem::OptProblemKind;

    #[test]
    fn solving_by_greedy_works() {
        let instance = Instance::from((vec![(1.0, 2.0), (2.0, 3.0), (2.0, 1.0)], 5.0));
        let solution = instance.run(Greedy);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }

    #[test]
    fn solving_by_dp_works() {
        let instance = Instance::from((vec![(1, 2), (2, 3), (2, 1)], 5));
        let solution = instance.run(DynamicProgram);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }
}
