use crate::algorithm::Algorithm;
use crate::knapsack::{DefaultItem, Instance, Item, Solution};

pub struct Greedy;

impl<I> Algorithm<Instance<I>> for Greedy
where
    I: Item,
{
    fn run(&self, instance: &Instance<I>) -> Solution {
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

#[cfg(test)]
mod test_super {
    use super::*;
    use crate::problem::OptProblemKind;

    #[test]
    fn solving_by_reduction_works() {
        let instance = Instance::from((vec![(1.0, 2.0), (2.0, 3.0), (2.0, 1.0)], 5.0));
        let solution = instance.run(Greedy);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }
}
