use complex::Complex;

#[derive(Clone, Debug)]
pub struct Point {
    loc: Complex,
    value: Complex,
    pub iterations: u64,
    pub escaped: bool,
}

impl Point {
    pub fn new(x:f64, y:f64) -> Point {
        Point {
            loc: Complex::new(x, y),
            value: Complex::new(x,y),
            iterations: 0,
            escaped: Complex::new(x,y).escaped(),
        }
    }

    pub fn iterate(self) -> Point {
        if !self.escaped {
            let new_value = self.value * self.value + self.loc;
            return Point {
                loc: self.loc,
                iterations: self.iterations + 1,
                value: new_value,
                escaped: new_value.escaped(),
            }
        }
        self
    }

    pub fn iterate_n(self, n:u64) -> Point {
        let mut v = self;
        for _ in 0..n {
            if v.escaped {
                return v;
            }
            v = v.iterate();
        }
        v
    }

    pub fn iterate_to_n(self, n:u64) -> Point {
        let mut v = self;
        for _ in v.iterations..n {
            if v.escaped {
                return v;
            }
            v = v.iterate();
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::Point;
    use ::complex::Complex;

    #[test]
    fn two_is_escaped() {
        let two = Point::new(2.0,0.0);
        
        assert_eq!(two.escaped, true);
        assert_eq!(two.iterations, 0);

        let r = two.iterate_n(1_000_000);
        assert_eq!(r.iterations, 0);
    }

    #[test]
    fn zero_never_escapes() {
        let zero = Point::new(0.0,0.0);
        let target_count = 1_000_000;
        let r = zero.iterate_n(target_count);

        assert_eq!(r.escaped, false);
        assert_eq!(r.iterations, target_count);
    }

    #[test]
    fn iterate_to_works() {
        let zero = Point::new(0.0,0.0);
        let ten = zero.iterate_n(10);
        let target_count = 1_000_000;
        let r = ten.iterate_to_n(target_count);

        assert_eq!(r.escaped, false);
        assert_eq!(r.iterations, target_count);
    }

    #[test]
    fn one_escapes() {
        let i = Point::new(1.0, 0.0);
        let target_count = 1_000_000;
        let r = i.iterate_n(target_count);

        assert_eq!(r.escaped, true);
        assert_eq!(r.iterations, 1);
    }

    #[test]
    fn iterates_correctly() {
        let c = Point::new(-1.0, 0.5);
        let one = c.iterate();

        assert_eq!(one.escaped, false);
        assert_eq!(one.iterations, 1);
        assert_eq!(one.value, Complex::new(-0.25, -0.5));
    }
}
