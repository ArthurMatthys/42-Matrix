use std::fmt;

pub struct Vector<K, const N: usize>([K; N]);

impl<K: std::fmt::Display, const N: usize> Vector<K, N> {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    pub fn size(&self) -> usize {
        N
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(item: [K; N]) -> Self {
        Vector(item)
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
