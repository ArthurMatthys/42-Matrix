use std::ops::Add;

use crate::model::{matrix::Matrix, scalar::Scalar};

impl<S, const M: usize> Matrix<S, M, M>
where
    S: Scalar + Add<Output = S>,
{
    pub fn _trace(self) -> S {
        let mut iter = self.0.into_iter();
        let first = iter.next().expect("But contain at leat one line")[0];
        iter.enumerate().fold(first, |acc, (i, v)| acc + v[i + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_00() {
        let u = Matrix([[1., 0.], [0., 1.]]);
        assert!(u._trace() == 2.);
    }
    #[test]
    fn test_trace_01() {
        let u = Matrix([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert!(u._trace() == 9.);
    }
    #[test]
    fn test_trace_02() {
        let u = Matrix([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert!(u._trace() == -21.);
    }
    #[test]
    fn test_trace_03() {
        let u = Matrix([[0., 0.], [0., 0.]]);
        assert!(u._trace() == 0.);
    }
    #[test]
    fn test_trace_04() {
        let u = Matrix([[1., 2.], [3., 4.]]);
        assert!(u._trace() == 5.);
    }
    #[test]
    fn test_trace_05() {
        let u = Matrix([[8., -7.], [4., 2.]]);
        assert!(u._trace() == 10.);
    }
    #[test]
    fn test_trace_06() {
        let u = Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert!(u._trace() == 3.);
    }
}
