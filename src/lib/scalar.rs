use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

pub trait Scalar: Add + Sub + Mul + Div + PartialOrd + PartialEq + Copy + Display {
    fn norm(self) -> f32;
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(self) -> bool {
        self == Scalar::zero()
    }
}

impl Scalar for f32 {
    fn norm(self) -> f32 {
        self.abs()
    }
    fn one() -> f32 {
        1.0
    }
    fn zero() -> f32 {
        0.
    }
}
