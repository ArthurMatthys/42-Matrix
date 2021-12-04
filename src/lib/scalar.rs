use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: Add + Sub + Mul + Div + Clone + PartialOrd {
    fn norm(self) -> f32;
}

impl Scalar for f32 {
    fn norm(self) -> f32 {
        self.abs()
    }
}
