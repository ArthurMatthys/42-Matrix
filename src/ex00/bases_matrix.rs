use crate::model::{matrix::Matrix, scalar::Scalar};
use std::ops::{Add, Mul, Sub};

impl<S, const M: usize, const N: usize> Add for Matrix<S, M, N>
where
    S: Scalar + Add<Output = S>,
{
    type Output = Self;
    fn add(self, other: Matrix<S, M, N>) -> Self::Output {
        Matrix(
            self.0
                .zip(other.0)
                .map(|(vec1, vec2)| vec1.zip(vec2).map(|(v1, v2)| v1 + v2)),
        )
    }
}
impl<S, const M: usize, const N: usize> Sub for Matrix<S, M, N>
where
    S: Scalar + Sub<Output = S>,
{
    type Output = Self;
    fn sub(self, other: Matrix<S, M, N>) -> Self::Output {
        Matrix(
            self.0
                .zip(other.0)
                .map(|(vec1, vec2)| vec1.zip(vec2).map(|(v1, v2)| v1 - v2)),
        )
    }
}
impl<S, const M: usize, const N: usize> Mul<S> for Matrix<S, M, N>
where
    S: Scalar + Mul<Output = S>,
{
    type Output = Self;
    fn mul(self, coef: S) -> Self::Output {
        Matrix(self.0.map(|vec1| vec1.map(|v| v * coef)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_0() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        let v = Matrix([[7., 4.], [-2., 2.]]);
        let res = Matrix([[8., 6.], [1., 6.]]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_1() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        let v = Matrix([[0., 0.], [0., 0.]]);
        let res = Matrix([[0., 0.], [0., 0.]]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_2() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let v = Matrix([[0., 0.], [0., 0.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_3() {
        let u = Matrix([[1., 1.], [1., 1.]]);
        let v = Matrix([[1., 1.], [1., 1.]]);
        let res = Matrix([[2., 2.], [2., 2.]]);
        assert!(u + v == res);
    }
    #[test]
    fn test_add_4() {
        let u = Matrix([[21., 21.], [21., 21.]]);
        let v = Matrix([[21., 21.], [21., 21.]]);
        let res = Matrix([[42., 42.], [42., 42.]]);
        assert!(u + v == res);
    }

    #[test]
    fn test_sub_0() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        let v = Matrix([[7., 4.], [-2., 2.]]);
        let res = Matrix([[-6., -2.], [5., 2.]]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_1() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        let v = Matrix([[0., 0.], [0., 0.]]);
        let res = Matrix([[0., 0.], [0., 0.]]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_2() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let v = Matrix([[0., 0.], [0., 0.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_3() {
        let u = Matrix([[1., 1.], [1., 1.]]);
        let v = Matrix([[1., 1.], [1., 1.]]);
        let res = Matrix([[0., 0.], [0., 0.]]);
        assert!(u - v == res);
    }
    #[test]
    fn test_sub_4() {
        let u = Matrix([[21., 21.], [21., 21.]]);
        let v = Matrix([[21., 21.], [21., 21.]]);
        let res = Matrix([[0., 0.], [0., 0.]]);
        assert!(u - v == res);
    }

    #[test]
    fn test_scl_0() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        let k = 2.;
        let res = Matrix([[2., 4.], [6., 8.]]);
        assert!(u * k == res);
    }
    #[test]
    fn test_mul_1() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        let k = 0.;
        let res = Matrix([[0., 0.], [0., 0.]]);
        assert!(u * k == res);
    }
    #[test]
    fn test_mul_2() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let k = 1.;
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u * k == res);
    }
    #[test]
    fn test_mul_3() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        let k = 2.;
        let res = Matrix([[2., 4.], [6., 8.]]);
        assert!(u * k == res);
    }
    #[test]
    fn test_mul_4() {
        let u = Matrix([[21., 21.], [21., 21.]]);
        let k = 0.5;
        let res = Matrix([[10.5, 10.5], [10.5, 10.5]]);
        assert!(u * k == res);
    }
}
