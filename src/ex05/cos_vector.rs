use std::ops::{Add, Div, Mul};

use crate::lib::{scalar::Scalar, vector::Vector};

fn angle_cos<S, const N: usize>(v1: Vector<S, N>, v2: Vector<S, N>) -> f32
where
    S: Scalar + Add<Output = S> + Mul<Output = S> + Div<f32, Output = f32>,
{
    v1.clone().dot(v2.clone()) / (v1.clone().norm() * v2.clone().norm())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tes_angle_cos_00() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert!(angle_cos(u, v) == 1.);
    }
    #[test]
    fn tes_angle_cos_01() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert!(angle_cos(u, v) == 0.);
    }
    #[test]
    fn tes_angle_cos_02() {
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert!(angle_cos(u, v) == -1.0000001);
    }
    #[test]
    fn tes_angle_cos_03() {
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert!(angle_cos(u, v) == 1.);
    }
    #[test]
    fn tes_angle_cos_04() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        println!("{}", angle_cos(u, v));
    }
    #[test]
    fn tes_angle_cos_05() {
        let u = Vector::from([8., 7.]);
        let v = Vector::from([3., 2.]);
        assert!(angle_cos(u, v) == 0.99145424);
    }
    #[test]
    fn tes_angle_cos_06() {
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        assert!(angle_cos(u, v) == 1.0000001);
    }
    #[test]
    fn tes_angle_cos_07() {
        let u = Vector::from([4., 2.]);
        let v = Vector::from([1., 1.]);
        assert!(angle_cos(u, v) == 0.94868326);
    }
    #[test]
    fn tes_angle_cos_08() {
        let u = Vector::from([-7., 3.]);
        let v = Vector::from([6., 4.]);
        assert!(angle_cos(u, v) == -0.54626775);
    }
}
