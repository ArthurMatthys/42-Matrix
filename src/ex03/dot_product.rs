use std::ops::{Add, Mul};

use crate::lib::{scalar::Scalar, vector::Vector};

impl<S, const N: usize> Vector<S, N>
where
    S: Scalar + Add<Output = S> + Mul<Output = S>,
{
    pub fn dot(self, other: Self) -> S {
        let mut iter = std::array::IntoIter::new(self.0.zip(other.0).map(|(v1, v2)| v1 * v2));
        let first = iter.next().expect("Your vector should not be empty");
        iter.fold(first, |acc, next| acc + next)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::complex::Complex;

    use super::*;

    #[test]
    fn test_dot_00() {
        let u = Vector([0., 0.]);
        let v = Vector([1., 1.]);
        assert_eq!(u.dot(v), 0.);
    }
    #[test]
    fn test_dot_01() {
        let u = Vector([1., 1.]);
        let v = Vector([1., 1.]);
        assert_eq!(u.dot(v), 2.);
    }
    #[test]
    fn test_dot_02() {
        let u = Vector([-1., 6.]);
        let v = Vector([3., 2.]);
        assert_eq!(u.dot(v), 9.);
    }
    #[test]
    fn test_dot_03() {
        let u = Vector([0., 0.]);
        let v = Vector([0., 0.]);
        assert_eq!(u.dot(v), 0.);
    }
    #[test]
    fn test_dot_04() {
        let u = Vector([1., 0.]);
        let v = Vector([0., 0.]);
        assert_eq!(u.dot(v), 0.);
    }
    #[test]
    fn test_dot_05() {
        let u = Vector([1., 0.]);
        let v = Vector([1., 0.]);
        assert_eq!(u.dot(v), 1.);
    }
    #[test]
    fn test_dot_06() {
        let u = Vector([1., 0.]);
        let v = Vector([0., 1.]);
        assert_eq!(u.dot(v), 0.);
    }
    #[test]
    fn test_dot_07() {
        let u = Vector([4., 2.]);
        let v = Vector([2., 1.]);
        assert_eq!(u.dot(v), 10.);
    }
    #[test]
    fn test_dot_complex_00() {
        let u = Vector([Complex(4., 2.), Complex(2., 1.)]);
        let v = Vector([Complex(1., 0.), Complex(1., 0.)]);
        let res = Complex(6., 3.);
        assert!(u.dot(v) == res);
    }
    #[test]
    fn test_dot_complex_01() {
        let u = Vector([Complex(4., 2.), Complex(2., 1.)]);
        let v = Vector([Complex(2., 1.), Complex(4., 2.)]);
        let res = Complex(12., 16.);
        assert!(u.dot(v) == res);
    }
    #[test]
    fn test_dot_complex_02() {
        let u = Vector([Complex(4., -2.), Complex(2., -1.)]);
        let v = Vector([Complex(-2., 1.), Complex(4., -2.)]);
        let res = Complex(0., 0.);
        assert!(u.dot(v) == res);
    }
}
