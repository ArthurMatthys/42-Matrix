use std::ops::{Add, Mul};

use crate::lib::{matrix::Matrix, scalar::Scalar};

fn _linear_combination<S, const M: usize, const N: usize, const B: usize>(
    matrices: [Matrix<S, M, N>; B],
    coefs: [S; B],
) -> Matrix<S, M, N>
where
    S: Scalar + Add<Output = S> + Mul<Output = S>,
{
    let mut iter = matrices.zip(coefs).map(|(m, k)| m * k).into_iter();
    let first = iter
        .next()
        .expect("You need to have more than one element to do a linear combination");
    iter.fold(first, |acc, next| acc + next)
}

#[cfg(test)]
mod tests {
    use crate::lib::complex::Complex;

    use super::*;

    #[test]
    fn test_linear_combination_00() {
        let e1 = Matrix([[-42., 42.]]);
        let coefs = [-1.];
        let result = Matrix([[42., -42.]]);
        assert!(_linear_combination([e1], coefs) == result);
    }
    #[test]
    fn test_linear_combination_01() {
        let e1 = Matrix([[-42.]]);
        let e2 = Matrix([[-42.]]);
        let e3 = Matrix([[-42.]]);
        let coefs = [-1., 1., 0.];
        let result = Matrix([[0.]]);
        assert!(_linear_combination([e1, e2, e3], coefs) == result);
    }
    #[test]
    fn test_linear_combination_02() {
        let e1 = Matrix([[-42., 42.], [420., -420.]]);
        let e2 = Matrix([[1., 3.], [2., 4.]]);
        let e3 = Matrix([[10., 20.], [30., 40.]]);
        let coefs = [1., -10., -1.];
        let result = Matrix([[-62., -8.], [370., -500.]]);
        assert!(_linear_combination([e1, e2, e3], coefs) == result);
    }
    #[test]
    fn test_linear_combination_03() {
        let e1 = Matrix([
            [Complex(1., 1.), Complex(3., 3.)],
            [Complex(2., 2.), Complex(4., 4.)],
        ]);
        let e2 = Matrix([
            [Complex(4., 4.), Complex(2., 2.)],
            [Complex(3., 3.), Complex(1., 1.)],
        ]);
        let coefs = [Complex(1., 2.), Complex(0., 1.)];
        let result = Matrix([
            [Complex(-5., 7.), Complex(-5., 11.)],
            [Complex(-5., 9.), Complex(-5., 13.)],
        ]);
        assert!(_linear_combination([e1, e2], coefs) == result);
    }
    #[test]
    fn test_linear_combination_04() {
        let e1 = Matrix([
            [Complex(1., 1.), Complex(3., 3.)],
            [Complex(2., 2.), Complex(4., 4.)],
        ]);
        let e2 = Matrix([
            [Complex(4., 4.), Complex(2., 2.)],
            [Complex(3., 3.), Complex(1., 1.)],
        ]);
        let e3 = Matrix([
            [Complex(1., 1.), Complex(1., 1.)],
            [Complex(1., 1.), Complex(1., 1.)],
        ]);
        let coefs = [Complex(1., 2.), Complex(3., 4.), Complex(5., 6.)];
        let result = Matrix([
            [Complex(-6., 42.), Complex(-6., 34.)],
            [Complex(-6., 38.), Complex(-6., 30.)],
        ]);
        assert!(_linear_combination([e1, e2, e3], coefs) == result);
    }
    #[test]
    fn test_linear_combination_05() {
        let e1 = Matrix([
            [Complex(-42., 2.), Complex(42., -30.)],
            [Complex(420., 210.), Complex(-420., -210.)],
        ]);
        let e2 = Matrix([
            [Complex(-42., 2.), Complex(42., -30.)],
            [Complex(420., 210.), Complex(-420., -210.)],
        ]);
        let e3 = Matrix([
            [Complex(-42., 2.), Complex(42., -30.)],
            [Complex(420., 210.), Complex(-420., -210.)],
        ]);
        let coefs = [Complex(1., 6.), Complex(-10., 15.), Complex(-1., -42.)];
        let result = Matrix([
            [Complex(462., 862.), Complex(-1050., -582.)],
            [Complex(210., -10920.), Complex(-210., 10920.)],
        ]);
        assert!(_linear_combination([e1, e2, e3], coefs) == result);
    }
}
