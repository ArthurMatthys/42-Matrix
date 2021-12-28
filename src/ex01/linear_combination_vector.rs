use std::ops::{Add, Mul};

use crate::model::{scalar::Scalar, vector::Vector};

fn _linear_combination<S, const N: usize, const B: usize>(
    vecs: [Vector<S, N>; B],
    coefs: [S; B],
) -> Vector<S, N>
where
    S: Scalar + Add<Output = S> + Mul<Output = S>,
{
    let mut iter = vecs.zip(coefs).map(|(v, k)| v * k).into_iter();
    let first = iter
        .next()
        .expect("You need to have more than one element to do a linear combination");
    iter.fold(first, |acc, next| acc + next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_combination_00() {
        let e1 = Vector([1., 0., 0.]);
        let e2 = Vector([0., 1., 0.]);
        let e3 = Vector([0., 0., 1.]);
        let coefs = [10., -2., 0.5];
        let result = Vector([10., -2., 0.5]);
        assert!(_linear_combination([e1, e2, e3], coefs) == result);
    }
    #[test]
    fn test_linear_combination_01() {
        let e1 = Vector([-42., 42.]);
        let coefs = [-1.];
        let result = Vector([42., -42.]);
        assert!(_linear_combination([e1], coefs) == result);
    }
    #[test]
    fn test_linear_combination_02() {
        let e1 = Vector([-42.]);
        let e2 = Vector([-42.]);
        let e3 = Vector([-42.]);
        let coefs = [-1., 1., 0.];
        let result = Vector([0.]);
        assert!(_linear_combination([e1, e2, e3], coefs) == result);
    }
    #[test]
    fn test_linear_combination_03() {
        let e1 = Vector([-42., 42.]);
        let e2 = Vector([1., 3.]);
        let e3 = Vector([10., 20.]);
        let coefs = [1., -10., -1.];
        let result = Vector([-62., -8.]);
        assert!(_linear_combination([e1, e2, e3], coefs) == result);
    }
    #[test]
    fn test_linear_combination_04() {
        let e1 = Vector([-42., 100., -69.5]);
        let e2 = Vector([1., 3., 5.]);
        let coefs = [1., -10.];
        let result = Vector([-52., 70., -119.5]);
        assert!(_linear_combination([e1, e2], coefs) == result);
    }
}
