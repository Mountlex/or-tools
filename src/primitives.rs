use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;

pub trait Numeric: Add + Sum + PartialEq + Display + Clone {}

impl Numeric for f32 {}
impl Numeric for f64 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
