use crate::lib::vector::Vector;

impl<K: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign, const N: usize>
    Vector<K, N>
{
    fn add(&mut self, vec: &Self) {
        for i in 0..N {
            self.get_mut()[i] += vec.get()[i];
        }
    }
    fn sub(&mut self, vec: &Self) {
        for i in 0..N {
            self.get_mut()[i] -= vec.get()[i];
        }
    }
    fn scl(&mut self, a: K) {
        for i in 0..N {
            self.get_mut()[i] *= a;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::vector::Vector;

    #[test]
    fn test_add_0() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_1() {
        let mut u = Vector::from([0., 0.]);
        let v = Vector::from([0., 0.]);
        let res = Vector::from([0., 0.]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_2() {
        let mut u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        let res = Vector::from([1., 1.]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_3() {
        let mut u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        let res = Vector::from([2., 2.]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_4() {
        let mut u = Vector::from([21., 21.]);
        let v = Vector::from([21., 21.]);
        let res = Vector::from([42., 42.]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_5() {
        let mut u = Vector::from([-21., 21.]);
        let v = Vector::from([21., -21.]);
        let res = Vector::from([0., 0.]);
        u.add(&v);
        assert!(u == res);
    }
    #[test]
    fn test_add_6() {
        let mut u = Vector::from([0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        let v = Vector::from([9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);
        let res = Vector::from([9., 9., 9., 9., 9., 9., 9., 9., 9., 9.]);
        u.add(&v);
        assert!(u == res);
    }

    #[test]
    fn test_sub_0() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        u.sub(&v);
        assert!(u == res);
    }
    #[test]
    fn test_sub_1() {
        let mut u = Vector::from([0., 0.]);
        let v = Vector::from([0., 0.]);
        let res = Vector::from([0., 0.]);
        u.sub(&v);
        assert!(u == res);
    }
    #[test]
    fn test_sub_2() {
        let mut u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        let res = Vector::from([1., -1.]);
        u.sub(&v);
        assert!(u == res);
    }
    #[test]
    fn test_sub_3() {
        let mut u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        let res = Vector::from([0., 0.]);
        u.sub(&v);
        assert!(u == res);
    }
    #[test]
    fn test_sub_4() {
        let mut u = Vector::from([21., 21.]);
        let v = Vector::from([21., 21.]);
        let res = Vector::from([0., 0.]);
        u.sub(&v);
        assert!(u == res);
    }
    #[test]
    fn test_sub_5() {
        let mut u = Vector::from([-21., 21.]);
        let v = Vector::from([21., -21.]);
        let res = Vector::from([-42., 42.]);
        u.sub(&v);
        assert!(u == res);
    }
    #[test]
    fn test_sub_6() {
        let mut u = Vector::from([0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        let v = Vector::from([9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);
        let res = Vector::from([-9., -7., -5., -3., -1., 1., 3., 5., 7., 9.]);
        u.sub(&v);
        assert!(u == res);
    }

    #[test]
    fn test_scl_0() {
        let mut u = Vector::from([2., 3.]);
        let a = 2.;
        let res = Vector::from([4., 6.]);
        u.scl(a);
        assert!(u == res);
    }
    #[test]
    fn test_scl_1() {
        let mut u = Vector::from([0., 0.]);
        let a = 1.;
        let res = Vector::from([0., 0.]);
        u.scl(a);
        assert!(u == res);
    }
    #[test]
    fn test_scl_2() {
        let mut u = Vector::from([1., 0.]);
        let a = 1.;
        let res = Vector::from([1., 0.]);
        u.scl(a);
        assert!(u == res);
    }
    #[test]
    fn test_scl_3() {
        let mut u = Vector::from([1., 1.]);
        let a = 2.;
        let res = Vector::from([2., 2.]);
        u.scl(a);
        assert!(u == res);
    }
    #[test]
    fn test_scl_4() {
        let mut u = Vector::from([21., 21.]);
        let a = 2.;
        let res = Vector::from([42., 42.]);
        u.scl(a);
        assert!(u == res);
    }
    #[test]
    fn test_scl_5() {
        let mut u = Vector::from([42., 42.]);
        let a = 0.5;
        let res = Vector::from([21., 21.]);
        u.scl(a);
        assert!(u == res);
    }
}
