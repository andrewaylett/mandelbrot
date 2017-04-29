use complex::Complex;

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
        for i in 0..n {
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
        let mut zero = Point::new(0.0,0.0);
        let target_count = 1_000_000;
        let r = zero.iterate_n(target_count);

        assert_eq!(r.escaped, false);
        assert_eq!(r.iterations, target_count);
    }

    #[test]
    fn i_escapes() {
        let mut i = Point::new(0.0, 1.0);
        let target_count = 1_000_000;
        let r = i.iterate_n(target_count);

        assert_eq!(r.escaped, true);
        assert_eq!(r.iterations, 2);
    }
}
