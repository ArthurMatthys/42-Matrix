use std::ops::{Add, Mul};

use crate::model::{scalar::Scalar, vector::Vector};

impl<S, const N: usize> Vector<S, N>
where
    S: Scalar + Add<Output = S>,
{
    pub fn _norm_1(self) -> f32 {
        let mut iter = self.0.into_iter();
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
    pub fn _norm(self) -> f32 {
        let mut iter = self.0.map(|v| v.norm().powf(2.)).into_iter();
        let first = iter
            .next()
            .expect("Vector should contain at least one number");
        iter.fold(first, |acc, next| acc + next).sqrt()
    }
}
impl<S, const N: usize> Vector<S, N>
where
    S: Scalar,
{
    pub fn _norm_inf(self) -> f32 {
        let mut iter = self.0.map(|v| v.norm()).into_iter();
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
    use crate::model::complex::Complex;

    use super::*;

    #[test]
    fn test_norm_1_00() {
        let u = Vector([0., 0., 0.]);
        assert!(u._norm_1() == 0.);
    }
    #[test]
    fn test_norm_1_01() {
        let u = Vector([1., 2., 3.]);
        assert!(u._norm_1() == 6.);
    }
    #[test]
    fn test_norm_1_02() {
        let u = Vector([-1., -2.]);
        assert!(u._norm_1() == 3.);
    }
    #[test]
    fn test_norm_1_03() {
        let u = Vector([0.]);
        assert!(u._norm_1() == 0.);
    }
    #[test]
    fn test_norm_1_04() {
        let u = Vector([1.]);
        assert!(u._norm_1() == 1.);
    }
    #[test]
    fn test_norm_1_05() {
        let u = Vector([0., 0.]);
        assert!(u._norm_1() == 0.);
    }
    #[test]
    fn test_norm_1_06() {
        let u = Vector([1., 0.]);
        assert!(u._norm_1() == 1.);
    }
    #[test]
    fn test_norm_1_07() {
        let u = Vector([2., 1.]);
        assert!(u._norm_1() == 3.);
    }
    #[test]
    fn test_norm_1_08() {
        let u = Vector([4., 2.]);
        assert!(u._norm_1() == 6.);
    }
    #[test]
    fn test_norm_1_09() {
        let u = Vector([-4., -2.]);
        assert!(u._norm_1() == 6.);
    }
    #[test]
    fn test_norm_00() {
        let u = Vector([0., 0., 0.]);
        assert!(u._norm() == 0.);
    }
    #[test]
    fn test_norm_01() {
        let u = Vector([1., 2., 3.]);
        assert!(u._norm() == 3.74165738);
    }
    #[test]
    fn test_norm_02() {
        let u = Vector([-1., -2.]);
        assert!(u._norm() == 2.236067977);
    }
    #[test]
    fn test_norm_03() {
        let u = Vector([0.]);
        assert!(u._norm() == 0.);
    }
    #[test]
    fn test_norm_04() {
        let u = Vector([1.]);
        assert!(u._norm() == 1.);
    }
    #[test]
    fn test_norm_05() {
        let u = Vector([0., 0.]);
        assert!(u._norm() == 0.);
    }
    #[test]
    fn test_norm_06() {
        let u = Vector([1., 0.]);
        assert!(u._norm() == 1.);
    }
    #[test]
    fn test_norm_07() {
        let u = Vector([2., 1.]);
        assert!(u._norm() == 2.236067977);
    }
    #[test]
    fn test_norm_08() {
        let u = Vector([4., 2.]);
        assert!(u._norm() == 4.472135955);
    }
    #[test]
    fn test_norm_09() {
        let u = Vector([-4., -2.]);
        assert!(u._norm() == 4.472135955);
    }
    #[test]
    fn test_norm_inf_00() {
        let u = Vector([0., 0., 0.]);
        assert!(u._norm_inf() == 0.);
    }
    #[test]
    fn test_norm_inf_01() {
        let u = Vector([1., 2., 3.]);
        assert!(u._norm_inf() == 3.);
    }
    #[test]
    fn test_norm_inf_02() {
        let u = Vector([-1., -2.]);
        assert!(u._norm_inf() == 2.);
    }
    #[test]
    fn test_norm_inf_03() {
        let u = Vector([0.]);
        assert!(u._norm_inf() == 0.);
    }
    #[test]
    fn test_norm_inf_04() {
        let u = Vector([1.]);
        assert!(u._norm_inf() == 1.);
    }
    #[test]
    fn test_norm_inf_05() {
        let u = Vector([0., 0.]);
        assert!(u._norm_inf() == 0.);
    }
    #[test]
    fn test_norm_inf_06() {
        let u = Vector([1., 0.]);
        assert!(u._norm_inf() == 1.);
    }
    #[test]
    fn test_norm_inf_07() {
        let u = Vector([2., 1.]);
        assert!(u._norm_inf() == 2.);
    }
    #[test]
    fn test_norm_inf_08() {
        let u = Vector([4., 2.]);
        assert!(u._norm_inf() == 4.);
    }
    #[test]
    fn test_norm_inf_09() {
        let u = Vector([-4., -2.]);
        assert!(u._norm_inf() == 4.);
    }
    #[test]
    fn test_norm_1_complex_00() {
        let u = Vector([Complex(-4., 3.), Complex(6., 8.)]);
        assert!(u._norm_1() == 15.);
    }
    #[test]
    fn test_norm_1_complex_01() {
        let u = Vector([Complex(-4., 0.), Complex(-3., -4.)]);
        assert!(u._norm_1() == 9.);
    }
    #[test]
    fn test_norm_complex_00() {
        let u = Vector([Complex(-4., 3.), Complex(6., -8.)]);
        assert!(u._norm() == 11.18034);
    }
    #[test]
    fn test_norm_complex_01() {
        let u = Vector([Complex(-4., 0.), Complex(-3., -4.)]);
        assert!(u._norm() == 6.4031243);
    }
    #[test]
    fn test_norm_inf_complex_00() {
        let u = Vector([Complex(-4., 0.), Complex(0., -3.)]);
        assert!(u._norm_inf() == 4.);
    }
    #[test]
    fn test_norm_inf_complex_01() {
        let u = Vector([Complex(-4., 0.), Complex(-3., -4.)]);
        assert!(u._norm_inf() == 5.);
    }
}
