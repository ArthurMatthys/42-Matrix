use crate::lib::matrix::Matrix;
use std::ops::{AddAssign, MulAssign, SubAssign};

impl<K: Copy + AddAssign + SubAssign + MulAssign, const M: usize, const N: usize> Matrix<K, M, N> {
    fn add(&mut self, matrix: &Self) {
        for i in 0..M {
            for j in 0..N {
                self.get_mut()[i][j] += matrix.get()[i][j];
            }
        }
    }
    fn sub(&mut self, matrix: &Self) {
        for i in 0..M {
            for j in 0..N {
                self.get_mut()[i][j] -= matrix.get()[i][j];
            }
        }
    }
    fn scl(&mut self, a: K) {
        for i in 0..M {
            for j in 0..N {
                self.get_mut()[i][j] *= a;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::matrix::Matrix;

    #[test]
    fn test_add_0() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = Matrix::from([[8., 6.], [1., 6.]]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_1() {
        let mut u = Matrix::from([[0., 0.], [0., 0.]]);
        let v = Matrix::from([[0., 0.], [0., 0.]]);
        let res = Matrix::from([[0., 0.], [0., 0.]]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_2() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[0., 0.], [0., 0.]]);
        let res = Matrix::from([[1., 0.], [0., 1.]]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_3() {
        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        let v = Matrix::from([[0., 0.], [0., 0.]]);
        let res = Matrix::from([[1., 1.], [1., 1.]]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_4() {
        let mut u = Matrix::from([[21., 21.], [21., 21.]]);
        let v = Matrix::from([[21., 21.], [21., 21.]]);
        let res = Matrix::from([[42., 42.], [42., 42.]]);
        u.add(&v);
        assert!(u == res);
    }

    #[test]
    fn test_sub_0() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = Matrix::from([[-6., -2.], [5., 2.]]);
        u.sub(&v);
        assert!(u == res);
    }

    #[test]
    fn test_scl() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = 2.;
        let res = Matrix::from([[2., 4.], [6., 8.]]);
        u.scl(v);
        assert!(u == res);
    }
}
