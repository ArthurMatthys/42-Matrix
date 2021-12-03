use crate::lib::matrix::Matrix;
use std::ops::{Add, Mul, Sub};

pub fn lerp<
    K: Clone + Add<Output = K> + Mul<Output = K> + Sub<Output = K>,
    const M: usize,
    const N: usize,
>(
    u: Matrix<K, M, N>,
    v: Matrix<K, M, N>,
    t: K,
) -> Matrix<K, M, N> {
    u.clone() + (v - u.clone()) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_00() {
        let u = Matrix::from([[2., 1.], [3., 4.]]);
        let v = Matrix::from([[20., 10.], [30., 40.]]);
        let t = 0.5;
        let res = Matrix::from([[11., 5.5], [16.5, 22.]]);
        assert!(lerp(u, v, t) == res)
    }
}
