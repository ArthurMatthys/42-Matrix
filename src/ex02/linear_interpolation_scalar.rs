use std::ops::{Add, Mul, Sub};

fn lerp<K: Clone + Add<Output = K> + Sub<Output = K> + Mul<Output = K>>(u: K, v: K, t: K) -> K {
    u.clone() + (v - u.clone()) * t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lerp_00() {
        assert_eq!(lerp(0., 1., 0.), 0.);
    }
    #[test]
    fn test_lerp_01() {
        assert_eq!(lerp(0., 1., 1.), 1.);
    }
    #[test]
    fn test_lerp_02() {
        assert_eq!(lerp(0., 1., 0.5), 0.5);
    }
    #[test]
    fn test_lerp_03() {
        assert_eq!(lerp(21., 42., 0.3), 27.3);
    }
    #[test]
    fn test_lerp_04() {
        assert_eq!(lerp(0., 42., 0.5), 21.);
    }
    #[test]
    fn test_lerp_05() {
        assert_eq!(lerp(-42., 42., 0.5), 0.);
    }
}
