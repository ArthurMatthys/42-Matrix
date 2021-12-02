use std::fmt;

pub struct Matrix<K, const M: usize, const N: usize>([[K; N]; M]);

impl<K, const M: usize, const N: usize> Matrix<K, M, N> {
    /// Return the shape of the given Matrix
    pub fn shape(&self) -> (usize, usize) {
        (M, N)
    }

    /// Determine is a Matrix is square or not
    pub fn is_square(&self) -> bool {
        M == N
    }
}

impl<K, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K, M, N> {
    fn from(item: [[K; N]; M]) -> Self {
        Matrix(item)
    }
}

impl<K: fmt::Display, const M: usize, const N: usize> Matrix<K, M, N> {
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
