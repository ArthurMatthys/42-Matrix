use std::ops::{Add, Mul, Sub};

pub trait Scalar: Add + Sub + Mul + Clone + PartialOrd {
    fn norm(self) -> f32;
}

impl Scalar for f32 {
    fn norm(self) -> f32 {
        self.abs()
    }
}
