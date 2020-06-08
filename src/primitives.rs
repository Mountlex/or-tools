use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;

pub trait Zero {
    fn zero() -> Self;
}

pub trait NumericKind: Clone + Copy + Display + Into<f64> {}
pub trait Numeric:
    Add + Add<Output = Self> + PartialOrd + Sum + Clone + Copy + Display + Zero
{
}

impl NumericKind for f32 {}
impl NumericKind for i32 {}
impl NumericKind for f64 {}
impl NumericKind for i16 {}

impl Numeric for f32 {}
impl Numeric for i32 {}
impl Numeric for u32 {}
impl Numeric for usize {}
impl Numeric for f64 {}

impl Zero for f32 {
    fn zero() -> f32 {
        0.0
    }
}
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
impl Zero for usize {
    fn zero() -> usize {
        0
    }
}
