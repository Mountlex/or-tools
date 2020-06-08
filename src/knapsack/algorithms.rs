use crate::algorithm::Algorithm;
use crate::knapsack::{DefaultItem, Instance, Item, Solution};

pub struct Greedy;

impl Algorithm<Instance<DefaultItem<f32>>> for Greedy {
    fn run(&self, instance: Instance<DefaultItem<f32>>) -> Solution {
        let mut items = instance.items();
        items.sort_by(|b, a| {
            (a.cost() / a.weight())
                .partial_cmp(&(b.cost() / b.weight()))
                .unwrap()
        });
        let packed_items = items
            .iter()
            .enumerate()
            .take_while(|(index, _)| {
                items
                    .iter()
                    .take(index + 1)
                    .map(|i| *i.weight())
                    .sum::<f32>()
                    < *instance.bag_size()
            })
            .map(|(index, _)| index)
            .collect();
        Solution::Solved {
            packed_items: packed_items,
        }
    }
}
