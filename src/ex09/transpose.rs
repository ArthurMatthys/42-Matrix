use crate::lib::{matrix::Matrix, scalar::Scalar};

impl<S, const M: usize, const N: usize> Matrix<S, M, N>
where
    S: Scalar,
{
    pub fn transpose(&self) -> Matrix<S, N, M> {
        let mut res = [[<S as Scalar>::zero(); M]; N];
        for j in 0..N {
            for i in 0..M {
                res[j][i] = self.0[i][j];
            }
        }
        Matrix(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transpose_00() {
        let m = Matrix([[0., 0.], [0., 0.]]);
        let res = Matrix([[0., 0.], [0., 0.]]);
        assert!(m.transpose() == res);
    }
    #[test]
    fn test_transpose_01() {
        let m = Matrix([[1., 0.], [0., 1.]]);
        let res = Matrix([[1., 0.], [0., 1.]]);
        assert!(m.transpose() == res);
    }
    #[test]
    fn test_transpose_02() {
        let m = Matrix([[1., 2.], [3., 4.]]);
        let res = Matrix([[1., 3.], [2., 4.]]);
        assert!(m.transpose() == res);
    }
    #[test]
    fn test_transpose_03() {
        let m = Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let res = Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert!(m.transpose() == res);
    }
    #[test]
    fn test_transpose_04() {
        let m = Matrix([[1., 2.], [3., 4.], [5., 6.]]);
        let res = Matrix([[1., 3., 5.], [2., 4., 6.]]);
        assert!(m.transpose() == res);
    }
    #[test]
    fn test_transpose_05() {
        let m = Matrix([[1., 3., 5.], [2., 4., 6.]]);
        let res = Matrix([[1., 2.], [3., 4.], [5., 6.]]);
        assert!(m.transpose() == res);
    }
}
