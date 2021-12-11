use std::ops::{Div, Mul, Sub};

use crate::lib::{matrix::Matrix, scalar::Scalar};

impl<S, const M: usize, const N: usize> Matrix<S, M, N>
where
    S: Scalar + Div<Output = S> + Sub<Output = S> + Mul<Output = S>,
{
    fn row_echelon(self) -> Self {
        let mut res = [[<S as Scalar>::zero(); N]; M];
        let swap_lines = |res: &mut [[S; N]; M], l1: usize, l2: usize| {
            for j in 0..N {
                let tmp = res[l1][j].clone();
                res[l1][j] = res[l2][j];
                res[l2][j] = tmp;
            }
        };
        let find_first_row = |res: [[S; N]; M], line: usize, col: usize| {
            for i in line..M {
                if !res[i][col].is_zero() {
                    return Some(i);
                }
            }
            return None;
        };
        if M == 0 || N == 0 {
            panic!("Must contain at least one column and one line");
        }
        for i in 0..M {
            for j in 0..N {
                res[i][j] = self.0[i][j].clone();
            }
        }
        let mut actual_line = 0;
        for col in 0..N {
            if let Some(line) = find_first_row(res, actual_line, col) {
                if line != actual_line {
                    swap_lines(&mut res, actual_line, line);
                }
                let pivot = res[actual_line][col].clone();
                for j in col..N {
                    res[actual_line][j] = res[actual_line][j] / pivot;
                }
                for i in 0..M {
                    if i == actual_line {
                        continue;
                    }
                    let pivot = res[i][col].clone();
                    for j in col..N {
                        res[i][j] = res[i][j] - pivot * res[actual_line][j];
                    }
                }
                actual_line += 1;
            } else {
                continue;
            }
        }
        Matrix(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_echelon_00() {
        let u = Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let res = Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_01() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_02() {
        let u = Matrix([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        let res = Matrix([
            [1., 0.625, 0., 0., -12.166668],
            [0., 0., 1., 0., -3.6666667],
            [0., 0., 0., 1., 29.5],
        ]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_03() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        let res = Matrix([[0., 0.], [0., 0.]]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_04() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_05() {
        let u = Matrix([[4., 2.], [2., 1.]]);
        let res = Matrix([[1., 0.5], [0., 0.]]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_06() {
        let u = Matrix([[-7., 2.], [4., 8.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_07() {
        let u = Matrix([[1., 2.], [4., 8.]]);
        let res = Matrix([[1., 2.], [0., 0.]]);
        assert!(u.row_echelon() == res);
    }
    #[test]
    fn test_row_echelon_08() {
        let u = Matrix([
            [8., 5., -2., 4., 28.],
            [8., 5., 1., 4., 17.],
            [3., 2.5, 20., 4., -4.],
        ]);
        let res = Matrix([
            [1., 0., 0., -2., -59.],
            [0., 1., 0., 4., 98.53334],
            [0., 0., 1., 0., -3.6666667],
        ]);
        assert!(u.row_echelon() == res);
    }
}
