use crate::problem::OptProblemKind;

pub enum TheoreticGuarantee {
    Consistent,
    Inconsistent(String),
    Failed(String),
}

impl TheoreticGuarantee {
    pub fn is_correct(&self) -> bool {
        matches!(self, TheoreticGuarantee::Consistent)
    }
}

pub trait Algorithm<V>
where
    V: OptProblemKind,
{
    fn run(&self, instance: &V) -> V::Solution;
}

pub trait TheoreticValidation<V>
where
    V: OptProblemKind,
{
    fn validate(&self, instance: &V, solution: &V::Solution) -> TheoreticGuarantee;

    fn is_correct(&self, instance: &V, solution: &V::Solution) -> bool {
        self.validate(instance, solution).is_correct()
    }
}
