use std::ops::{Add, Mul, Sub};

use crate::lib::scalar::Scalar;

pub fn lerp<V: Clone + Add<Output = V> + Mul<S, Output = V> + Sub<Output = V>, S: Scalar>(
    u: V,
    v: V,
    t: S,
) -> V {
    u.clone() + (v - u) * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::complex::Complex;
    use crate::lib::matrix::Matrix;
    use crate::lib::vector::Vector;

    #[test]
    fn test_lerp_scalar_00() {
        assert_eq!(lerp(0., 1., 0.), 0.);
    }
    #[test]
    fn test_lerp_scalar_01() {
        assert_eq!(lerp(0., 1., 1.), 1.);
    }
    #[test]
    fn test_lerp_scalar_02() {
        assert_eq!(lerp(0., 1., 0.5), 0.5);
    }
    #[test]
    fn test_lerp_scalar_03() {
        assert_eq!(lerp(21., 42., 0.3), 27.3);
    }
    #[test]
    fn test_lerp_scalar_04() {
        assert_eq!(lerp(0., 42., 0.5), 21.);
    }
    #[test]
    fn test_lerp_scalar_05() {
        assert_eq!(lerp(-42., 42., 0.5), 0.);
    }
    #[test]
    fn test_lerp_r00() {
        let u = Vector([2., 1.]);
        let v = Vector([4., 2.]);
        let t = 0.3;
        let res = Vector([2.6, 1.3]);
        assert!(lerp(u, v, t) == res)
    }
    #[test]
    fn test_lerp_vector_01() {
        let u = Vector([-42., 42.]);
        let v = Vector([42., -42.]);
        let t = 0.5;
        let res = Vector([0., 0.]);
        assert!(lerp(u, v, t) == res)
    }
    #[test]
    fn test_lerp_matrix_00() {
        let u = Matrix([[2., 1.], [3., 4.]]);
        let v = Matrix([[20., 10.], [30., 40.]]);
        let t = 0.5;
        let res = Matrix([[11., 5.5], [16.5, 22.]]);
        assert!(lerp(u, v, t) == res)
    }
    #[test]
    fn test_lerp_complex_00() {
        assert!(lerp(Complex(1., 0.), Complex(0., 1.), Complex(0.5, 0.)) == Complex(0.5, 0.5))
    }
    #[test]
    fn test_lerp_complex_01() {
        let u = Vector([Complex(1., 0.), Complex(0., 1.)]);
        let v = Vector([Complex(0., 1.), Complex(1., 0.)]);
        let t = Complex(0.5, 0.);
        let res = Vector([Complex(0.5, 0.5), Complex(0.5, 0.5)]);
        assert!(lerp(u, v, t) == res)
    }
}
