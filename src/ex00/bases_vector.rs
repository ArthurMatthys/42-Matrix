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
    use crate::lib::{complex::Complex, vector::Vector};

    #[test]
    fn test_add_0() {
        let u = Vector([2., 3.]);
        let v = Vector([5., 7.]);
        let res = Vector([7., 10.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_1() {
        let u = Vector([0., 0.]);
        let v = Vector([0., 0.]);
        let res = Vector([0., 0.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_2() {
        let u = Vector([1., 0.]);
        let v = Vector([0., 1.]);
        let res = Vector([1., 1.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_3() {
        let u = Vector([1., 1.]);
        let v = Vector([1., 1.]);
        let res = Vector([2., 2.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_4() {
        let u = Vector([21., 21.]);
        let v = Vector([21., 21.]);
        let res = Vector([42., 42.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_5() {
        let u = Vector([-21., 21.]);
        let v = Vector([21., -21.]);
        let res = Vector([0., 0.]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_6() {
        let u = Vector([0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        let v = Vector([9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);
        let res = Vector([9., 9., 9., 9., 9., 9., 9., 9., 9., 9.]);
        assert!(u + v == res);
    }

    #[test]
    fn test_sub_0() {
        let u = Vector([2., 3.]);
        let v = Vector([5., 7.]);
        let res = Vector([-3., -4.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_1() {
        let u = Vector([0., 0.]);
        let v = Vector([0., 0.]);
        let res = Vector([0., 0.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_2() {
        let u = Vector([1., 0.]);
        let v = Vector([0., 1.]);
        let res = Vector([1., -1.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_3() {
        let u = Vector([1., 1.]);
        let v = Vector([1., 1.]);
        let res = Vector([0., 0.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_4() {
        let u = Vector([21., 21.]);
        let v = Vector([21., 21.]);
        let res = Vector([0., 0.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_5() {
        let u = Vector([-21., 21.]);
        let v = Vector([21., -21.]);
        let res = Vector([-42., 42.]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_6() {
        let u = Vector([0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        let v = Vector([9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);
        let res = Vector([-9., -7., -5., -3., -1., 1., 3., 5., 7., 9.]);
        assert!(u - v == res);
    }

    #[test]
    fn test_scl_0() {
        let u = Vector([2., 3.]);
        let a = 2.;
        let res = Vector([4., 6.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_1() {
        let u = Vector([0., 0.]);
        let a = 1.;
        let res = Vector([0., 0.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_2() {
        let u = Vector([1., 0.]);
        let a = 1.;
        let res = Vector([1., 0.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_3() {
        let u = Vector([1., 1.]);
        let a = 2.;
        let res = Vector([2., 2.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_4() {
        let u = Vector([21., 21.]);
        let a = 2.;
        let res = Vector([42., 42.]);
        assert!(u * a == res);
    }
    #[test]
    fn test_scl_5() {
        let u = Vector([42., 42.]);
        let a = 0.5;
        let res = Vector([21., 21.]);
        assert!(u * a == res);
    }

    #[test]
    fn test_complex_add_0() {
        let u = Vector([Complex(2., 4.), Complex(3., -2.)]);
        let v = Vector([Complex(5., 3.), Complex(1., -8.)]);
        let res = Vector([Complex(7., 7.), Complex(4., -10.)]);
        assert!(u + v == res);
    }
    #[test]
    fn test_complex_add_1() {
        let u = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let v = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let res = Vector([Complex(0., 0.), Complex(0., 0.)]);
        assert!(u + v == res);
    }
    #[test]
    fn test_complex_add_2() {
        let u = Vector([Complex(21., 21.), Complex(21., 21.)]);
        let v = Vector([Complex(21., 21.), Complex(21., 21.)]);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u + v == res);
    }
    #[test]
    fn test_complex_add_3() {
        let u = Vector([Complex(42., 42.), Complex(42., 42.)]);
        let v = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u + v == res);
    }

    #[test]
    fn test_complex_sub_0() {
        let u = Vector([Complex(2., 4.), Complex(3., -2.)]);
        let v = Vector([Complex(5., 3.), Complex(1., -8.)]);
        let res = Vector([Complex(-3., 1.), Complex(2., 6.)]);
        assert!(u - v == res);
    }
    #[test]
    fn test_complex_sub_1() {
        let u = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let v = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let res = Vector([Complex(0., 0.), Complex(0., 0.)]);
        assert!(u - v == res);
    }
    #[test]
    fn test_complex_sub_2() {
        let u = Vector([Complex(21., 21.), Complex(21., 21.)]);
        let v = Vector([Complex(21., 21.), Complex(21., 21.)]);
        let res = Vector([Complex(0., 0.), Complex(0., 0.)]);
        assert!(u - v == res);
    }
    #[test]
    fn test_complex_sub_3() {
        let u = Vector([Complex(42., 42.), Complex(42., 42.)]);
        let v = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u - v == res);
    }
    #[test]
    fn test_complex_sub_4() {
        let u = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let v = Vector([Complex(42., 42.), Complex(42., 42.)]);
        let res = Vector([Complex(-42., -42.), Complex(-42., -42.)]);
        assert!(u - v == res);
    }
    #[test]
    fn test_complex_sub_5() {
        let u = Vector([Complex(84., 84.), Complex(84., 84.)]);
        let v = Vector([Complex(42., 42.), Complex(42., 42.)]);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u - v == res);
    }
    #[test]
    fn test_complex_sub_6() {
        let u = Vector([Complex(21., 21.), Complex(21., 21.)]);
        let v = Vector([Complex(-21., -21.), Complex(-21., -21.)]);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u - v == res);
    }

    #[test]
    fn test_complex_scl_0() {
        let u = Vector([Complex(21., 21.), Complex(21., 21.)]);
        let a = Complex(2., 0.);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_1() {
        let u = Vector([Complex(0., 0.), Complex(0., 0.)]);
        let a = Complex(2., 0.);
        let res = Vector([Complex(0., 0.), Complex(0., 0.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_2() {
        let u = Vector([Complex(1., 1.), Complex(1., 1.)]);
        let a = Complex(15., 0.);
        let res = Vector([Complex(15., 15.), Complex(15., 15.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_3() {
        let u = Vector([Complex(84., 84.), Complex(84., 84.)]);
        let a = Complex(0.5, 0.);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_4() {
        let u = Vector([Complex(-42., -42.), Complex(-42., -42.)]);
        let a = Complex(-1., 0.);
        let res = Vector([Complex(42., 42.), Complex(42., 42.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_5() {
        let u = Vector([Complex(-42., -42.), Complex(-42., -42.)]);
        let a = Complex(1., 1.);
        let res = Vector([Complex(0., -84.), Complex(0., -84.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_6() {
        let u = Vector([Complex(1., 1.), Complex(1., 1.)]);
        let a = Complex(1., 1.);
        let res = Vector([Complex(0., 2.), Complex(0., 2.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_7() {
        let u = Vector([Complex(42., 42.), Complex(42., 42.)]);
        let a = Complex(1., 1.);
        let res = Vector([Complex(0., 84.), Complex(0., 84.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_8() {
        let u = Vector([Complex(42., 42.), Complex(42., 42.)]);
        let a = Complex(0.5, 0.5);
        let res = Vector([Complex(0., 42.), Complex(0., 42.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_9() {
        let u = Vector([Complex(42., 42.), Complex(42., 42.)]);
        let a = Complex(0.5, 2.);
        let res = Vector([Complex(-63., 105.), Complex(-63., 105.)]);
        assert!(u * a == res);
    }
    #[test]
    fn test_complex_scl_10() {
        let u = Vector([Complex(42., 6.), Complex(-15., 21.)]);
        let a = Complex(6.5, 2.3);
        let res = Vector([Complex(259.2, 135.6), Complex(-145.8, 102.)]);
        assert!(u * a == res);
    }
}
