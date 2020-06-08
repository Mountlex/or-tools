use crate::algorithm::Algorithm;
use crate::knapsack::{DefaultItem, Instance, Item, Solution};
use crate::primitives::Numeric;
use crate::problem::OptProblemKind;

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

pub struct SimpleDP;

impl<I, C> Algorithm<Instance<I, C, i32>> for SimpleDP
where
    I: Item<C, i32>,
    C: Numeric,
{
    fn run(&self, instance: &Instance<I, C, i32>) -> Solution {
        let mut values = vec![
            vec![C::zero(); (*instance.bag_size() + 1) as usize];
            instance.number_of_items() + 1
        ];

        for (i, item) in instance.items().iter().enumerate() {
            for j in 1..*instance.bag_size() + 1 {
                values[i + 1][j as usize] = if *item.weight() > j {
                    values[i][j as usize]
                } else {
                    max!(
                        values[i][j as usize],
                        values[i][(j - item.weight()) as usize] + *item.cost()
                    )
                }
            }
        }

        let mut packed: Vec<usize> = Vec::new();
        let mut left_weight = *instance.bag_size() as usize;

        for (i, item) in instance.items().iter().enumerate().rev() {
            if values[i + 1][left_weight] != values[i][left_weight] {
                packed.push(i);
                left_weight -= *item.weight() as usize;
            }
        }
        packed.sort();
        Solution::Solved {
            packed_items: packed,
        }
    }
}

pub struct FPTAS {
    eps: f64,
}

impl FPTAS {
    fn new(eps: f64) -> Self {
        FPTAS { eps }
    }
}
impl<I, C> Algorithm<Instance<I, C, i32>> for FPTAS
where
    I: Item<C, i32>,
    C: Numeric + Into<f64>,
{
    fn run(&self, instance: &Instance<I, C, i32>) -> Solution {
        let highest_cost: f64 = (instance
            .items()
            .iter()
            .map(|item| *item.cost())
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap())
        .into();

        let k = self.eps * (highest_cost / instance.number_of_items() as f64);

        let updated_items: Vec<DefaultItem<i32>> = instance
            .items()
            .iter()
            .map(|item| DefaultItem {
                cost: ((*item.cost()).into() / k).floor() as i32,
                weight: *item.weight(),
            })
            .collect();

        let updated_instance = Instance::new(updated_items, *instance.bag_size());
        updated_instance.run(SimpleDP)
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
        let solution = instance.run(SimpleDP);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }

    #[test]
    fn solving_by_fptas_works() {
        let instance = Instance::from((vec![(1, 2), (2, 3), (2, 1)], 5));
        let solution = instance.run(FPTAS::new(0.5));
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }
}
