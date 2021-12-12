use std::ops::{Div, Mul, Sub};

use crate::lib::{matrix::Matrix, scalar::Scalar};

impl<S, const M: usize, const N: usize> Matrix<S, M, N>
where
    S: Scalar + Div<Output = S> + Sub<Output = S> + Mul<Output = S>,
{
    pub fn rank(self) -> usize {
        let row_echelon_matrix = self.row_echelon();
        let mut rank = 0;
        for i in 0..M {
            for j in 0..N {
                if !row_echelon_matrix.0[i][j].is_zero() {
                    rank += 1;
                    break;
                }
            }
        }
        rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_00() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let res = 3;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_01() {
        let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
        let res = 2;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_02() {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
        let res = 3;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_03() {
        let u = Matrix::from([[0., 0.], [0., 0.]]);
        let res = 0;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_04() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let res = 2;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_05() {
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let res = 2;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_06() {
        let u = Matrix::from([[1., 1.], [1., 1.]]);
        let res = 1;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_07() {
        let u = Matrix::from([[0., 1.], [1., 0.]]);
        let res = 2;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_08() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let res = 2;
        assert!(u.rank() == res);
    }
    #[test]
    fn test_rank_09() {
        let u = Matrix::from([[-7., 5.], [4., 6.]]);
        let res = 2;
        assert!(u.rank() == res);
    }
}
