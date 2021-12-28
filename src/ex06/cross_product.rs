use std::ops::{Add, Mul, Sub};

use crate::model::{scalar::Scalar, vector::Vector};

pub fn _cross_product<S>(u: Vector<S, 3>, v: Vector<S, 3>) -> Vector<S, 3>
where
    S: Scalar + Mul<Output = S> + Sub<Output = S> + Add<Output = S>,
{
    Vector([
        u.0[1] * v.0[2] - u.0[2] * v.0[1],
        u.0[2] * v.0[0] - u.0[0] * v.0[2],
        u.0[0] * v.0[1] - u.0[1] * v.0[0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product_00() {
        let u = Vector([0., 0., 1.]);
        let v = Vector([1., 0., 0.]);
        let res = Vector([0., 1., 0.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_01() {
        let u = Vector([1., 2., 3.]);
        let v = Vector([4., 5., 6.]);
        let res = Vector([-3., 6., -3.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_02() {
        let u = Vector([4., 2., -3.]);
        let v = Vector([-2., -5., 16.]);
        let res = Vector([17., -58., -16.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_03() {
        let u = Vector([0., 0., 0.]);
        let v = Vector([0., 0., 0.]);
        let res = Vector([0., 0., 0.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_04() {
        let u = Vector([1., 0., 0.]);
        let v = Vector([0., 0., 0.]);
        let res = Vector([0., 0., 0.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_05() {
        let u = Vector([1., 0., 0.]);
        let v = Vector([0., 1., 0.]);
        let res = Vector([0., 0., 1.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_06() {
        let u = Vector([8., 7., -4.]);
        let v = Vector([3., 2., 1.]);
        let res = Vector([15., -20., -5.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_07() {
        let u = Vector([1., 1., 1.]);
        let v = Vector([0., 0., 0.]);
        let res = Vector([0., 0., 0.]);
        assert!(_cross_product(u, v) == res);
    }
    #[test]
    fn test_cross_product_08() {
        let u = Vector([1., 1., 1.]);
        let v = Vector([1., 1., 1.]);
        let res = Vector([0., 0., 0.]);
        assert!(_cross_product(u, v) == res);
    }
}
