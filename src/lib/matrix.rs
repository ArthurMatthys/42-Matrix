use std::fmt;

#[derive(Clone)]
pub struct Matrix<K, const M: usize, const N: usize>(pub(crate) [[K; N]; M]);

impl<K, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn new(matrix: [[K; N]; M]) -> Self {
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

    pub fn get(&self) -> &[[K; N]; M] {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut [[K; N]; M] {
        &mut self.0
    }
}

impl<K: fmt::Display, const M: usize, const N: usize> fmt::Display for Matrix<K, M, N> {
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

impl<K, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K, M, N> {
    fn from(item: [[K; N]; M]) -> Self {
        Matrix(item)
    }
}

impl<K: std::cmp::PartialEq, const M: usize, const N: usize> PartialEq for Matrix<K, M, N> {
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
