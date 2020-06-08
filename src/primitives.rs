use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, Div};

pub trait Zero {
    fn zero() -> Self;
}

pub trait Numeric: Display + Clone + Into<f64> + Into<f64> {}

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
impl Zero for i64 {
    fn zero() -> i64 {
        0
    }
}

impl Numeric for f32 {}
impl Numeric for i32 {}
