use complex::Complex;
use num::{Zero, One};

#[derive(Clone, Debug)]
pub struct Point<Unit: One + Zero> {
    loc: Complex<Unit>,
    value: Complex<Unit>,
    pub iterations: u64,
    pub escaped: bool,
}

impl<Unit: Zero + One> Point<Unit> {
    pub fn origin() -> Point<Unit> {
        Point::from_unit(Zero::zero(), Zero::zero())
    }

    pub fn from_unit(x:Unit, y:Unit) -> Point<Unit> {
        Point::new(Complex::new(x, y))
    }

    pub fn new(c:Complex<Unit>  ) -> Point<Unit> {
        let escaped = c.escaped();
        Point {
            loc: c.clone(),
            value: c,
            iterations: 0,
            escaped,
        }
    }

    pub fn from_integers(x: i64, y:i64) -> Point<Unit> {
        Point::from_unit(From::from(x), From::from(y))
    }

    pub fn iterate(&mut self) {
        if !self.escaped {
            self.value *= &self.value;
            self.value += &self.loc;
            self.escaped = self.value.escaped();
            self.iterations += 1;
        }
    }

    pub fn iterate_n(self, n:u64) -> Point<Unit> {
        let mut v = self.clone();
        for _ in 0..n {
            if v.escaped {
                return v;
            }
            v.iterate();
        }
        v
    }

    pub fn iterate_to_n(self, n:u64) -> Point<Unit> {
        let mut v = self.clone();
        for _ in v.iterations..n {
            if v.escaped {
                return v;
            }
            v.iterate();
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::Point;
    use crate::complex::Complex;

    #[test]
    fn two_is_escaped() {
        let two = Point::from_integers(2,0);
        
        assert_eq!(two.escaped, true);
        assert_eq!(two.iterations, 0);

        let r = two.iterate_n(1_000_000);
        assert_eq!(r.iterations, 0);
    }

    #[test]
    fn zero_never_escapes() {
        let zero = Point::origin();
        let target_count = 1_000_000;
        let r = zero.iterate_n(target_count);

        assert_eq!(r.escaped, false);
        assert_eq!(r.iterations, target_count);
    }

    #[test]
    fn iterate_to_works() {
        let zero = Point::origin();
        let ten = zero.iterate_n(10);
        let target_count = 1_000_000;
        let r = ten.iterate_to_n(target_count);

        assert_eq!(r.escaped, false);
        assert_eq!(r.iterations, target_count);
    }

    #[test]
    fn one_escapes() {
        let i = Point::from_integers(1, 0);
        let target_count = 1_000_000;
        let r = i.iterate_n(target_count);

        assert_eq!(r.escaped, true);
        assert_eq!(r.iterations, 1);
    }

    #[test]
    fn iterates_correctly() {
        let mut c: Point<f64> = Point::from_unit(-1f64, 0.5f64);
        c.iterate();

        assert_eq!(c.escaped, false);
        assert_eq!(c.iterations, 1);
        assert_eq!(c.value, Complex::new(-0.25f64, -0.5f64));
    }
}
