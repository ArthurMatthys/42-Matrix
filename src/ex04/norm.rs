use std::ops::{Add, Mul};

use crate::lib::{scalar::Scalar, vector::Vector};

impl<S, const N: usize> Vector<S, N>
where
    S: Scalar + Add<Output = S>,
{
    fn norm_1(self) -> f32 {
        let mut iter = std::array::IntoIter::new(self.0);
        let first = iter
            .next()
            .expect("Vector should contain at least one number");
        iter.fold(first.norm(), |acc, next| acc + next.norm())
    }
}
impl<S, const N: usize> Vector<S, N>
where
    S: Scalar + Add<Output = S> + Mul<Output = S>,
{
    fn norm(self) -> f32 {
        let mut iter = std::array::IntoIter::new(self.0.map(|v| v.norm().powf(2.)));
        let first = iter
            .next()
            .expect("Vector should contain at least one number");
        iter.fold(first, |acc, next| acc + next)
    }
}
impl<S, const N: usize> Vector<S, N>
where
    S: Scalar,
{
    fn norm_inf(self) -> f32 {
        let mut iter = std::array::IntoIter::new(self.0.map(|v| v.norm()));
        let mut max = iter
            .next()
            .expect("Vector should contain at least one number");
        iter.for_each(|v| {
            if v > max {
                max = v
            }
        });
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_1_00() {
        let u = Vector([0., 0., 0.]);
        assert!(u.norm_1() == 0.);
    }
    #[test]
    fn test_norm_1_01() {
        let u = Vector([1., 2., 3.]);
        assert!(u.norm_1() == 6.);
    }
    #[test]
    fn test_norm_1_02() {
        let u = Vector([-1., -2.]);
        assert!(u.norm_1() == -3.);
    }
}
