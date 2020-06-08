use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign};

pub trait Zero {
    fn zero() -> Self;
}

pub trait Numeric:
    Add
    + AddAssign
    + Add<Output = Self>
    + PartialOrd
    + PartialEq
    + Sum
    + Clone
    + Copy
    + Display
    + Zero
    + Into<f64>
{
}

impl Numeric for f64 {}
impl Numeric for i32 {}
impl Numeric for u32 {}

impl Zero for f64 {
    fn zero() -> f64 {
        0.0
    }
}
impl Zero for i32 {
    fn zero() -> i32 {
        0
    }
}
impl Zero for u32 {
    fn zero() -> u32 {
        0
    }
}
