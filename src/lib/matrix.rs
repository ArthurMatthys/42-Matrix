use std::fmt;

use super::scalar::Scalar;

#[derive(Clone)]
pub struct Matrix<S, const M: usize, const N: usize>(pub(crate) [[S; N]; M]);

impl<S: fmt::Display, const M: usize, const N: usize> fmt::Display for Matrix<S, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..M {
            write!(f, "[ ")?;
            for j in 0..N {
                if j == 0 {
                    write!(f, "{}", self.0[i][j])?;
                } else {
                    write!(f, ", {}", self.0[i][j])?;
                }
            }
            writeln!(f, " ]")?;
        }
        Ok(())
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
