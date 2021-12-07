use crate::lib::{scalar::Scalar, vector::Vector};
use std::ops::{Add, Mul, Sub};

impl<S, const N: usize> Add for Vector<S, N>
where
    S: Scalar + Add<Output = S>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0.zip(other.0).map(|(v1, v2)| v1 + v2))
    }
}

impl<S, const N: usize> Sub for Vector<S, N>
where
    S: Scalar + Sub<Output = S>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.zip(other.0).map(|(v1, v2)| v1 - v2))
    }
}

impl<S, const N: usize> Mul<S> for Vector<S, N>
where
    S: Scalar + Mul<Output = S>,
{
    type Output = Self;
    fn mul(self, coef: S) -> Self::Output {
        Self(self.0.map(|v| v * coef))
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::vector::Vector;

    #[test]
    fn test_add_0() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_1() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([0., 0.]);
        let res = Vector::from([0., 0.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_2() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        let res = Vector::from([1., 1.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_3() {
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        let res = Vector::from([2., 2.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_4() {
        let u = Vector::from([21., 21.]);
        let v = Vector::from([21., 21.]);
        let res = Vector::from([42., 42.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_5() {
        let u = Vector::from([-21., 21.]);
        let v = Vector::from([21., -21.]);
        let res = Vector::from([0., 0.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_6() {
        let u = Vector::from([0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        let v = Vector::from([9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);
        let res = Vector::from([9., 9., 9., 9., 9., 9., 9., 9., 9., 9.]);
        assert!(u + v == res);
    }

    #[test]
    fn test_sub_0() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_1() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([0., 0.]);
        let res = Vector::from([0., 0.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_2() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        let res = Vector::from([1., -1.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_3() {
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        let res = Vector::from([0., 0.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_4() {
        let u = Vector::from([21., 21.]);
        let v = Vector::from([21., 21.]);
        let res = Vector::from([0., 0.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_5() {
        let u = Vector::from([-21., 21.]);
        let v = Vector::from([21., -21.]);
        let res = Vector::from([-42., 42.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_6() {
        let u = Vector::from([0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        let v = Vector::from([9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);
        let res = Vector::from([-9., -7., -5., -3., -1., 1., 3., 5., 7., 9.]);
        assert!(u - v == res);
    }

    #[test]
    fn test_scl_0() {
        let u = Vector::from([2., 3.]);
        let a = 2.;
        let res = Vector::from([4., 6.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_1() {
        let u = Vector::from([0., 0.]);
        let a = 1.;
        let res = Vector::from([0., 0.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_2() {
        let u = Vector::from([1., 0.]);
        let a = 1.;
        let res = Vector::from([1., 0.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_3() {
        let u = Vector::from([1., 1.]);
        let a = 2.;
        let res = Vector::from([2., 2.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_4() {
        let u = Vector::from([21., 21.]);
        let a = 2.;
        let res = Vector::from([42., 42.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_5() {
        let u = Vector::from([42., 42.]);
        let a = 0.5;
        let res = Vector::from([21., 21.]);
        assert!(u * a == res);
    }
}
