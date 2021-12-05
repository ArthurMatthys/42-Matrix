use std::{fmt, mem::MaybeUninit};

use super::scalar::Scalar;

#[derive(Clone)]
pub struct Matrix<S, const M: usize, const N: usize>(pub(crate) [[S; N]; M]);

impl<S, const M: usize, const N: usize> Matrix<S, M, N>
where
    S: Scalar,
    MaybeUninit<S>: Copy,
{
    pub fn new(matrix: [[S; N]; M]) -> Self {
        Self(matrix)
    }
    /// Return the shape of the given Matrix
    pub fn shape(&self) -> (usize, usize) {
        (M, N)
    }

    /// Determine is a Matrix is square or not
    pub fn is_square(&self) -> bool {
        M == N
    }
}

impl<S: fmt::Display, const M: usize, const N: usize> fmt::Display for Matrix<S, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(for i in 0..M {
            write!(f, "[ ")?;
            for j in 0..N {
                if j == 0 {
                    write!(f, "{}", self.0[i][j])?;
                } else {
                    write!(f, ", {}", self.0[i][j])?;
                }
            }
            write!(f, " ]\n")?;
        })
    }
}

impl<S, const M: usize, const N: usize> From<[[S; N]; M]> for Matrix<S, M, N> {
    fn from(item: [[S; N]; M]) -> Self {
        Matrix(item)
    }
}

impl<S, const M: usize, const N: usize> PartialEq for Matrix<S, M, N>
where
    S: Scalar,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                if self.0[i][j] != other.0[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_shape_0() {
        let my_matrix = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(my_matrix.shape(), (2, 2));
    }
    #[test]
    fn test_shape_1() {
        let my_matrix = Matrix::from([[1., 2., 3.], [4., 5., 5.]]);
        assert_eq!(my_matrix.shape(), (2, 3));
    }
    #[test]
    fn test_shape_2() {
        let my_matrix = Matrix::from([[1., 2.], [3., 4.], [5., 6.]]);
        assert_eq!(my_matrix.shape(), (3, 2));
    }
    #[test]
    fn test_square_0() {
        let my_matrix = Matrix::from([[1., 2.], [3., 4.]]);
        assert!(my_matrix.is_square());
    }
    #[test]
    fn test_square_1() {
        let my_matrix = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
        assert!(!my_matrix.is_square());
    }
}
