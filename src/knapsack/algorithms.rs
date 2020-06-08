use crate::algorithm::*;
use crate::knapsack::{DefaultItem, Instance, Item, Solution};
use crate::primitives::Numeric;
use crate::problem::{OptProblemKind, SolutionKind};
use crate::program::LpSolver;
use crate::reduction::Reduction;

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

impl<I, C, W> Algorithm<Instance<I, C, W>> for Greedy
where
    I: Item<C, W>,
    C: Numeric,
    W: Numeric,
{
    fn run(&self, instance: &Instance<I, C, W>) -> Solution {
        let mut indexed_items: Vec<(usize, &I)> = instance.items().iter().enumerate().collect();
        indexed_items.sort_by(|(_, b), (_, a)| {
            ((*a.cost()).into() / (*a.weight()).into())
                .partial_cmp(&((*b.cost()).into() / (*b.weight()).into()))
                .unwrap()
        });
        let mut weight: W = W::zero();
        let mut next: usize = 0;
        let mut packed: Vec<usize> = Vec::new();
        for (index, item) in indexed_items {
            if weight + *item.weight() > (*instance.bag_size()) {
                next = index;
                break;
            } else {
                weight += *item.weight();
                packed.push(index);
            }
        }
        packed.sort();

        if packed.len() == instance.number_of_items() || weight > *instance.items()[next].weight() {
            Solution::Solved {
                packed_items: packed,
            }
        } else {
            Solution::Solved {
                packed_items: vec![next],
            }
        }
    }
}

impl<I, C, W> TheoreticValidation<Instance<I, C, W>> for Greedy
where
    I: Item<C, W>,
    C: Numeric,
    W: Numeric,
{
    fn validate(&self, instance: &Instance<I, C, W>, solution: &Solution) -> TheoreticGuarantee {
        let optimal_solution = instance.solve_by_reduction(&LpSolver::CBC);
        match (solution.cost(instance), optimal_solution.cost(instance)) {
            (Some(alg), Some(opt)) => {
                if alg.into() < 0.5 * opt.into() {
                    TheoreticGuarantee::Inconsistent(format!("Greedy algorithm did not achieve its theoretical approximation ratio: {} < 0.5 * {}", alg, opt))
                } else {
                    TheoreticGuarantee::Consistent
                }
            }
            _ => TheoreticGuarantee::Failed(format!(
                "Error: Cost of optimal solution and algorithm could not have been computed!"
            )),
        }
    }
}

pub struct SimpleDP;

impl<I, C> Algorithm<Instance<I, C, u32>> for SimpleDP
where
    I: Item<C, u32>,
    C: Numeric,
{
    fn run(&self, instance: &Instance<I, C, u32>) -> Solution {
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

impl<I, C> Algorithm<Instance<I, C, u32>> for FPTAS
where
    I: Item<C, u32>,
    C: Numeric,
{
    fn run(&self, instance: &Instance<I, C, u32>) -> Solution {
        let highest_cost: C = instance
            .items()
            .iter()
            .map(|item| *item.cost())
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();

        let k = self.eps * (highest_cost.into() / instance.number_of_items() as f64);

        let updated_items: Vec<DefaultItem<u32>> = instance
            .items()
            .iter()
            .map(|item| DefaultItem {
                cost: ((*item.cost()).into() / k).floor() as u32,
                weight: *item.weight(),
            })
            .collect();

        let updated_instance = Instance::new(updated_items, *instance.bag_size());
        updated_instance.run(SimpleDP)
    }
}

impl<I, C> TheoreticValidation<Instance<I, C, u32>> for FPTAS
where
    I: Item<C, u32>,
    C: Numeric,
{
    fn validate(&self, instance: &Instance<I, C, u32>, solution: &Solution) -> TheoreticGuarantee {
        let optimal_solution = instance.run(SimpleDP);
        match (solution.cost(instance), optimal_solution.cost(instance)) {
            (Some(alg), Some(opt)) => {
                if alg.into() < (1.0 - self.eps) * opt.into() {
                    TheoreticGuarantee::Inconsistent(
                        format!("FPTAS (eps = {}) did not achieve its theoretical approximation ratio: {} < {} * {}", self.eps, alg, 1.0 - self.eps, opt),
                    )
                } else {
                    TheoreticGuarantee::Consistent
                }
            }
            _ => TheoreticGuarantee::Failed(format!(
                "Error: Cost of optimal solution and algorithm could not have been computed!"
            )),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};

    #[test]
    fn solving_by_greedy_works() {
        let instance = Instance::from((vec![(1, 2), (2, 3), (2, 1)], 5));
        let solution = instance.run(Greedy);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }

    #[test]
    fn random_validation_greedy_alg() {
        let mut rng = thread_rng();
        let costs: Vec<u32> = rng.sample_iter(Uniform::new(0, 100)).take(30).collect();
        let weights: Vec<u32> = rng.sample_iter(Uniform::new(1, 100)).take(30).collect();
        let size = rng.sample(Uniform::new(400, 700));
        let instance = Instance::from((costs.into_iter().zip(weights.into_iter()).collect(), size));
        let solution = instance.run(Greedy);
        assert!(solution.is_solved());
        let validation = Greedy.validate(&instance, &solution);
        assert!(validation.is_correct());
    }

    #[test]
    fn solving_by_simple_dp_works() {
        let instance = Instance::from((vec![(1, 2), (2, 3), (2, 1)], 5));
        let solution = instance.run(SimpleDP);
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }

    #[test]
    fn random_validation_simple_dp_alg() {
        let mut rng = thread_rng();
        let costs: Vec<u32> = rng.sample_iter(Uniform::new(0, 100)).take(30).collect();
        let weights: Vec<u32> = rng.sample_iter(Uniform::new(1, 100)).take(30).collect();
        let size: u32 = rng.sample(Uniform::new(400, 700));
        let instance = Instance::from((costs.into_iter().zip(weights.into_iter()).collect(), size));
        let dp_solution = instance.run(SimpleDP);
        let ilp_solution = instance.solve_by_reduction(&LpSolver::CBC);
        assert!(dp_solution.is_solved());
        assert!(ilp_solution.is_solved());
        assert_eq!(ilp_solution.cost(&instance), dp_solution.cost(&instance));
    }

    #[test]
    fn solving_by_fptas_works() {
        let instance = Instance::from((vec![(1, 2), (2, 3), (2, 1)], 5));
        let solution = instance.run(FPTAS::new(0.5));
        assert!(solution.is_solved());
        let items = solution.as_solution().unwrap();
        assert_eq!(items, vec![1, 2])
    }

    #[test]
    fn random_validation_fptas_alg() {
        let mut rng = thread_rng();
        let costs: Vec<u32> = rng.sample_iter(Uniform::new(0, 100)).take(30).collect();
        let weights: Vec<u32> = rng.sample_iter(Uniform::new(1, 100)).take(30).collect();
        let size: u32 = rng.sample(Uniform::new(400, 700));
        let eps: f64 = rng.sample(Uniform::new(0.05, 0.95));
        let instance = Instance::from((costs.into_iter().zip(weights.into_iter()).collect(), size));
        let solution = instance.run(FPTAS::new(eps));
        assert!(solution.is_solved());
        let validation = FPTAS::new(eps).validate(&instance, &solution);
        assert!(validation.is_correct());
    }
}
