use crate::problem::OptProblemKind;
use std::ops::Add;
use std::iter::Sum;

struct Instance<I> where I : Item {
    items: Vec<I>,
    size: I::Weight
}

impl <I> Instance<I> where I: Item {
    pub fn number_of_items(&self) -> usize {
        self.items.len()
    }

    pub fn size(&self) -> I::Weight {
        self.size
    }
}

impl <I> OptProblemKind for Instance<I> where I : Item {
    type Solution = Solution<I>;
    type Cost = I::Cost;

    fn cost(solution: &Self::Solution) -> I::Cost {
        solution.into_iter().map(|item| *item.cost()).sum()
    }
}

struct Solution<I> where I : Item  {
    packed_items: Vec<I>,
}

impl <'a, I> IntoIterator for &'a Solution<I> where I : Item {
    type Item = &'a I;
    type IntoIter = std::slice::Iter<'a, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.packed_items.iter()
    }
}

trait Item {
    type Weight: Add + Sum + PartialEq + Copy;
    type Cost : Add + Sum + PartialEq + Copy;

    fn weight(&self) -> &Self::Weight;
    fn cost(&self) -> &Self::Cost;
}

struct DefaultItem<T> where T: Add + Sum + PartialEq {
    weight: T,
    cost: T
}


impl <T> Item for DefaultItem<T> where T: Add + Sum + PartialEq + Copy {
    type Weight = T;
    type Cost = T;

    fn weight(&self) -> &Self::Weight { &self.weight }
    fn cost(&self) -> &Self::Cost { &self.cost }
}

impl <T> From<(Vec<(T,T)>, T)> for Instance<DefaultItem<T>> where T: Add + Sum + PartialEq + Copy {
    fn from(input: (Vec<(T,T)>, T)) -> Self {
        let items: Vec<DefaultItem<T>> = input.0.iter().map(|&(cost, weight)| DefaultItem::from((cost, weight))).collect();
        Instance {
            items: items,
            size: input.1
        }
    }
}
impl <T> From<(T, T)> for DefaultItem<T> where T: Add + Sum + PartialEq + Copy {
    fn from(input: (T, T)) -> Self {
        DefaultItem {
            cost: input.0,
            weight: input.1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_instance_from_works() {
        let instance = Instance::from((vec![(1,2),(2,3),(3,4)], 5));
        assert_eq!(3, instance.number_of_items());
        assert_eq!(5, instance.size());
    }
}


