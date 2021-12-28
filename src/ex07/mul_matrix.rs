use std::ops::{Add, Mul};

use crate::model::{matrix::Matrix, scalar::Scalar, vector::Vector};

impl<S, const M: usize, const N: usize> Mul<Vector<S, N>> for Matrix<S, M, N>
where
    S: Scalar + Mul<Output = S> + Add<Output = S>,
{
    type Output = Vector<S, M>;
    fn mul(self, rhs: Vector<S, N>) -> Vector<S, M> {
        Vector(self.0.map(|v| Vector(v).dot(rhs.clone())))
    }
}

impl<S, const M: usize, const N: usize, const P: usize> Mul<Matrix<S, N, P>> for Matrix<S, M, N>
where
    S: Scalar + Mul<Output = S> + Add<Output = S>,
    [S; N]: Copy,
{
    type Output = Matrix<S, M, P>;
    fn mul(self, rhs: Matrix<S, N, P>) -> Self::Output {
        let transpose = rhs.transpose();
        Matrix(
            self.0
                .map(|v1| transpose.clone().0.map(|v2| Vector(v1).dot(Vector(v2)))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_vector_00() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let v = Vector([4., 2.]);
        let res = Vector([4., 2.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_01() {
        let u = Matrix([[2., 0.], [0., 2.]]);
        let v = Vector([4., 2.]);
        let res = Vector([8., 4.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_02() {
        let u = Matrix([[2., -2.], [-2., 2.]]);
        let v = Vector([4., 2.]);
        let res = Vector([4., -4.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_03() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let v = Vector([4.42143, -543.2313]);
        let res = Vector([4.42143, -543.2313]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_04() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        let v = Vector([4., 2.]);
        let res = Vector([0., 0.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_05() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        let v = Vector([4.42143, -543.2313]);
        let res = Vector([0., 0.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_06() {
        let u = Matrix([[1., 1.], [1., 1.]]);
        let v = Vector([4., 2.]);
        let res = Vector([6., 6.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_07() {
        let u = Matrix([[2., 0.], [0., 2.]]);
        let v = Vector([2., 1.]);
        let res = Vector([4., 2.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_vector_08() {
        let u = Matrix([[0.5, 0.], [0., 0.5]]);
        let v = Vector([8., 4.]);
        let res = Vector([4., 2.]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_matrix_00() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let v = Matrix([[1., 0.], [0., 1.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_matrix_01() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let v = Matrix([[2., 1.], [4., 2.]]);
        let res = Matrix([[2., 1.], [4., 2.]]);
        assert!(u * v == res);
    }
    #[test]
    fn test_mul_matrix_02() {
        let u = Matrix([[3., -5.], [6., 8.]]);
        let v = Matrix([[2., 1.], [4., 2.]]);
        let res = Matrix([[-14., -7.], [44., 22.]]);
        assert!(u * v == res);
    }
}
