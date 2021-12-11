use std::ops::{Add, Mul, Sub};

use crate::lib::{matrix::Matrix, scalar::Scalar};

pub fn vec_determinant<S>(v: Vec<Vec<S>>, size: usize) -> S
where
    S: Scalar + Sub<Output = S> + Mul<Output = S> + Add<Output = S>,
{
    if size == 1 {
        return v[0][0];
    } else if size == 2 {
        return v[0][0] * v[1][1] - v[0][1] * v[1][0];
    } else {
        let mut sub_matrix: Vec<Vec<S>> = vec![];
        for _ in 0..size - 1 {
            let mut line: Vec<S> = vec![];
            for _ in 0..size - 1 {
                line.push(<S as Scalar>::zero());
            }
            sub_matrix.push(line);
        }
        let mut res = <S as Scalar>::zero();
        for col in 0..size {
            for i in 1..size {
                for j in 0..size {
                    if j == col {
                        continue;
                    }
                    if j > col {
                        sub_matrix[i - 1][j - 1] = v[i][j];
                    } else {
                        sub_matrix[i - 1][j] = v[i][j];
                    }
                }
            }
            if col % 2 == 1 {
                res = res - v[0][col] * vec_determinant(sub_matrix.clone(), size - 1);
            } else {
                res = res + v[0][col] * vec_determinant(sub_matrix.clone(), size - 1);
            }
        }
        return res;
    }
}

impl<S, const M: usize> Matrix<S, M, M>
where
    S: Scalar + Sub<Output = S> + Mul<Output = S> + Add<Output = S>,
{
    pub fn determinant(self) -> S {
        let mut sub_matrix = vec![];
        for i in 0..M {
            let mut line: Vec<S> = vec![];
            for j in 0..M {
                line.push(self.0[i][j]);
            }
            sub_matrix.push(line);
        }
        return vec_determinant(sub_matrix, M);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_determinant_00() {
        let u = Matrix([[1., -1.], [-1., 1.]]);
        let res = 0.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_01() {
        let u = Matrix([[1.]]);
        let res = 1.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_02() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        let res = 0.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_03() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let res = 1.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_04() {
        let u = Matrix([[2., 0.], [0., 2.]]);
        let res = 4.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_05() {
        let u = Matrix([[1., 1.], [1., 1.]]);
        let res = 0.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_06() {
        let u = Matrix([[0., 1.], [1., 0.]]);
        let res = -1.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_07() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        let res = -2.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_08() {
        let u = Matrix([[-7., 5.], [4., 6.]]);
        let res = -62.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_09() {
        let u = Matrix([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        let res = 8.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_10() {
        let u = Matrix([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        let res = -174.;
        assert!(u.determinant() == res);
    }
    #[test]
    fn test_determinant_11() {
        let u = Matrix([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        let res = 1032.;
        assert!(u.determinant() == res);
    }
}
