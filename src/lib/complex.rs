use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use super::scalar::Scalar;

#[derive(Clone, Copy)]
pub struct Complex<S>(pub(crate) S, pub(crate) S);

impl<S> Add for Complex<S>
where
    S: Scalar + Add<Output = S>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<S> Sub for Complex<S>
where
    S: Scalar + Sub<Output = S>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<S> Mul for Complex<S>
where
    S: Scalar + Mul<Output = S> + Add<Output = S> + Sub<Output = S>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}
impl<S> Div for Complex<S>
where
    S: Scalar + Mul<Output = S> + Add<Output = S> + Sub<Output = S> + Div<Output = S>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Complex(
            (self.0 * rhs.0 + self.1 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1),
            (self.1 * rhs.0 - self.0 * rhs.1) / (rhs.0 * rhs.0 + rhs.1 * rhs.1),
        )
    }
}

impl<S> fmt::Display for Complex<S>
where
    S: Scalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(write!(f, "{} {}i", self.0, self.1)?)
    }
}
impl<S> PartialEq for Complex<S>
where
    S: Scalar,
{
    fn eq(&self, rhs: &Complex<S>) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1
    }
}

impl<S> Scalar for Complex<S>
where
    S: Scalar + Div<Output = S> + Add<Output = S> + Sub<Output = S> + Mul<Output = S>,
{
    fn norm(self) -> f32 {
        (self.0.norm().powf(2.) + self.1.norm().powf(2.)).sqrt()
    }
    fn one() -> Complex<S> {
        Complex(<S as Scalar>::one(), <S as Scalar>::zero())
    }
    fn zero() -> Complex<S> {
        Complex(<S as Scalar>::zero(), <S as Scalar>::zero())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_add_00() {
        let c1 = Complex(1., 1.);
        let c2 = Complex(-1., -1.);
        let res = Complex(0., 0.);
        assert!(c1 + c2 == res);
    }
    #[test]
    pub fn test_add_01() {
        let c1 = Complex(21., 21.);
        let c2 = Complex(21., 21.);
        let res = Complex(42., 42.);
        assert!(c1 + c2 == res);
    }
    #[test]
    pub fn test_add_02() {
        let c1 = Complex(1., 1.);
        let c2 = Complex(-1., -1.);
        let res = Complex(0., 0.);
        assert!(c1 + c2 == res);
    }
}
