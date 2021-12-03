use std::fmt;

#[derive(Clone)]
pub struct Vector<K, const N: usize>(pub(crate) [K; N]);

impl<K, const N: usize> Vector<K, N> {
    pub fn new(vector: [K; N]) -> Self {
        Self(vector)
    }
    pub fn size(&self) -> usize {
        N
    }
    pub fn get(&self) -> &[K; N] {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut [K; N] {
        &mut self.0
    }
}

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
        write!(f, " ]\n")?;
        return Ok(());
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

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn test_shape_0() {
        let my_vector = Vector::from([1., 2., 3., 4.]);
        assert_eq!(my_vector.size(), 4);
    }
    #[test]
    fn test_shape_1() {
        let my_vector = Vector::from([1., 2.]);
        assert_eq!(my_vector.size(), 2);
    }
}
