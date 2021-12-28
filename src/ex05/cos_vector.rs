use std::ops::{Add, Div, Mul};

use crate::model::{scalar::Scalar, vector::Vector};

fn _angle_cos<S, const N: usize>(v1: Vector<S, N>, v2: Vector<S, N>) -> f32
where
    S: Scalar + Add<Output = S> + Mul<Output = S> + Div<f32, Output = f32>,
{
    v1.clone().dot(v2.clone()) / (v1._norm() * v2._norm())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tes_angle_cos_00() {
        let u = Vector([1., 0.]);
        let v = Vector([1., 0.]);
        assert!(_angle_cos(u, v) == 1.);
    }
    #[test]
    fn tes_angle_cos_01() {
        let u = Vector([1., 0.]);
        let v = Vector([0., 1.]);
        assert!(_angle_cos(u, v) == 0.);
    }
    #[test]
    fn tes_angle_cos_02() {
        let u = Vector([-1., 1.]);
        let v = Vector([1., -1.]);
        assert!(_angle_cos(u, v) == -1.0000001);
    }
    #[test]
    fn tes_angle_cos_03() {
        let u = Vector([2., 1.]);
        let v = Vector([4., 2.]);
        assert!(_angle_cos(u, v) == 1.);
    }
    #[test]
    fn tes_angle_cos_04() {
        let u = Vector([1., 2., 3.]);
        let v = Vector([4., 5., 6.]);
        println!("{}", _angle_cos(u, v));
    }
    #[test]
    fn tes_angle_cos_05() {
        let u = Vector([8., 7.]);
        let v = Vector([3., 2.]);
        assert!(_angle_cos(u, v) == 0.99145424);
    }
    #[test]
    fn tes_angle_cos_06() {
        let u = Vector([1., 1.]);
        let v = Vector([1., 1.]);
        assert!(_angle_cos(u, v) == 1.0000001);
    }
    #[test]
    fn tes_angle_cos_07() {
        let u = Vector([4., 2.]);
        let v = Vector([1., 1.]);
        assert!(_angle_cos(u, v) == 0.94868326);
    }
    #[test]
    fn tes_angle_cos_08() {
        let u = Vector([-7., 3.]);
        let v = Vector([6., 4.]);
        assert!(_angle_cos(u, v) == -0.54626775);
    }
}
