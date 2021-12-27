use std::fmt;

#[derive(Clone)]
pub struct Vector<K, const N: usize>(pub(crate) [K; N]);

impl<K: std::fmt::Display, const N: usize> fmt::Display for Vector<K, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for i in 0..N {
            if i == 0 {
                write!(f, "{}", self.0[i])?;
            } else {
                write!(f, ", {}", self.0[i])?;
            }
        }
        writeln!(f, " ]")?;
        Ok(())
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(item: [K; N]) -> Self {
        Vector(item)
    }
}

impl<K: std::cmp::PartialEq, const N: usize> PartialEq for Vector<K, N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}
