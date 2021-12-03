use crate::lib::matrix::Matrix;
use std::ops::{Add, Mul};

fn linear_combination<
    K: Clone + Add<Output = K> + Mul<Output = K>,
    const M: usize,
    const N: usize,
    const B: usize,
>(
    matrices: [Matrix<K, M, N>; B],
    coefs: [K; B],
) -> Matrix<K, M, N> {
    let mut iter = std::array::IntoIter::new(matrices.zip(coefs).map(|(m, k)| m * k));
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
        let e1 = Matrix::from([[-42., 42.]]);
        let coefs = [-1.];
        let result = Matrix::from([[42., -42.]]);
        assert!(linear_combination([e1], coefs) == result);
    }
    #[test]
    fn test_linear_combination_01() {
        let e1 = Matrix::from([[-42.]]);
        let e2 = Matrix::from([[-42.]]);
        let e3 = Matrix::from([[-42.]]);
        let coefs = [-1., 1., 0.];
        let result = Matrix::from([[0.]]);
        assert!(linear_combination([e1, e2, e3], coefs) == result);
    }
    #[test]
    fn test_linear_combination_02() {
        let e1 = Matrix::from([[-42., 42.], [420., -420.]]);
        let e2 = Matrix::from([[1., 3.], [2., 4.]]);
        let e3 = Matrix::from([[10., 20.], [30., 40.]]);
        let coefs = [1., -10., -1.];
        let result = Matrix::from([[-62., -8.], [370., -500.]]);
        assert!(linear_combination([e1, e2, e3], coefs) == result);
    }
}
