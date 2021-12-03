use crate::lib::vector::Vector;
use std::ops::{Add, Mul, Sub};

pub fn lerp<K: Clone + Add<Output = K> + Mul<Output = K> + Sub<Output = K>, const N: usize>(
    u: Vector<K, N>,
    v: Vector<K, N>,
    t: K,
) -> Vector<K, N> {
    u.clone() + (v - u.clone()) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_00() {
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        let t = 0.3;
        let res = Vector::from([2.6, 1.3]);
        assert!(lerp(u, v, t) == res)
    }
    #[test]
    fn test_lerp_01() {
        let u = Vector::from([-42., 42.]);
        let v = Vector::from([42., -42.]);
        let t = 0.5;
        let res = Vector::from([0., 0.]);
        assert!(lerp(u, v, t) == res)
    }
}
