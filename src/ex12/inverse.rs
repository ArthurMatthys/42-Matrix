use std::ops::{Add, Div, Mul, Sub};

use crate::lib::{matrix::Matrix, scalar::Scalar};

impl<S, const M: usize> Matrix<S, M, M>
where
    S: Scalar + Div<Output = S> + Sub<Output = S> + Mul<Output = S> + Add<Output = S>,
{
    pub fn inverse(self) -> Result<Self, String> {
        if self.clone().determinant().is_zero() {
            return Err("Determinant is null. No inverse for this matrix".to_string());
        }
        let mut cpy_matrix = [[<S as Scalar>::zero(); M]; M];
        let mut res = [[<S as Scalar>::zero(); M]; M];
        let swap_lines = |matrix: &mut [[S; M]; M], l1: usize, l2: usize| {
            for j in 0..M {
                let tmp = matrix[l1][j].clone();
                matrix[l1][j] = matrix[l2][j];
                matrix[l2][j] = tmp;
            }
        };
        let find_first_row = |matrix: [[S; M]; M], line: usize, col: usize| {
            for i in line..M {
                if !matrix[i][col].is_zero() {
                    return Some(i);
                }
            }
            return None;
        };
        let add_columns = |matrix: &mut [[S; M]; M], l1: usize, l2: usize, coef: S| {
            for j in 0..M {
                matrix[l2][j] = matrix[l2][j] - coef * matrix[l1][j];
            }
        };
        let div_column = |matrix: &mut [[S; M]; M], line: usize, coef: S| {
            for j in 0..M {
                matrix[line][j] = matrix[line][j] / coef;
            }
        };
        for i in 0..M {
            for j in 0..M {
                cpy_matrix[i][j] = self.0[i][j].clone();
                if i == j {
                    res[i][j] = <S as Scalar>::one();
                }
            }
        }
        let mut actual_line = 0;
        for col in 0..M {
            if let Some(line) = find_first_row(res, actual_line, col) {
                if line != actual_line {
                    swap_lines(&mut cpy_matrix, actual_line, line);
                    swap_lines(&mut res, actual_line, line);
                }
                let coef_line = cpy_matrix[actual_line][col].clone();
                div_column(&mut cpy_matrix, actual_line, coef_line);
                div_column(&mut res, actual_line, coef_line);
                for i in 0..M {
                    if i == actual_line {
                        continue;
                    }
                    let pivot = cpy_matrix[i][col].clone();
                    add_columns(&mut cpy_matrix, actual_line, i, pivot);
                    add_columns(&mut res, actual_line, i, pivot);
                }
                actual_line += 1;
            } else {
                continue;
            }
        }
        Ok(Matrix(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_00() {
        let u = Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let res = Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert!(u.inverse() == Ok(res));
    }
    #[test]
    fn test_inverse_01() {
        let u = Matrix([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        let res = Matrix([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
        assert!(u.inverse() == Ok(res));
    }
    #[test]
    fn test_inverse_02() {
        let u = Matrix([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        let res = Matrix([
            [0.64942527, 0.09770115, -0.6551724],
            [-0.7816092, -0.12643678, 0.9655172],
            [0.14367816, 0.07471265, -0.20689656],
        ]);
        assert!(u.inverse() == Ok(res));
    }
    #[test]
    fn test_inverse_03() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u.inverse() == Ok(res));
    }
    #[test]
    fn test_inverse_04() {
        let u = Matrix([[2., 0.], [0., 2.]]);
        let res = Matrix([[0.5, 0.], [0., 0.5]]);
        assert!(u.inverse() == Ok(res));
    }
    #[test]
    fn test_inverse_05() {
        let u = Matrix([[0.5, 0.], [0., 0.5]]);
        let res = Matrix([[2., 0.], [0., 2.]]);
        assert!(u.inverse() == Ok(res));
    }
    #[test]
    fn test_inverse_06() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        let res = Matrix([[-2., 1.], [1.5, -0.5]]);
        assert!(u.inverse() == Ok(res));
    }
    #[test]
    fn test_inverse_07() {
        let u = Matrix([[1., 2.], [4., 8.]]);
        assert!(u.inverse().is_err());
    }
}
