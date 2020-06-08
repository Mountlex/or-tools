use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, Div};

pub trait Zero {
    fn zero() -> Self;
}

pub trait NumericKind: Clone + Copy + Display + Into<f64> {}

impl NumericKind for f32 {}
impl NumericKind for i32 {}
impl NumericKind for f64 {}
impl NumericKind for i16 {}
