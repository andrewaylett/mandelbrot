use crate::complex::{Complex, Fix2x61, FixError};
use anyhow::{Context, Error};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    loc: Complex,
    value: Complex,
    pub iterations: u64,
    pub escaped: bool,
}

impl Point {
    pub const ORIGIN: Point = Point::new(Complex::new(Fix2x61::zero(), Fix2x61::zero()));

    pub fn from_parts(x: &Fix2x61, y: &Fix2x61) -> Point {
        Point::new(Complex::new(*x, *y))
    }

    pub const fn new(c: Complex) -> Point {
        let escaped = false; //c.escaped();
        Point {
            loc: c,
            value: c,
            iterations: 0,
            escaped,
        }
    }

    pub fn iterate(&mut self) -> Result<(), Error> {
        if !self.escaped {
            let iterated = self.value.iterate_mandelbrot(&self.loc);
            if let Err(FixError::Escaped) = iterated {
                self.escaped = true;
            } else {
                self.value = iterated
                    .with_context(|| format!("Iterating, value before: {:?}", self.value))?
            }
            self.iterations += 1;
        }
        Ok(())
    }

    pub fn iterate_n(self, n: u64) -> Result<Point, Error> {
        let mut v = self;
        for i in 0..n {
            if v.escaped {
                return Ok(v);
            }
            v.iterate().with_context(|| {
                format!(
                    "Iterate n {} of {}, iteration {}, value {:?}",
                    i, n, v.iterations, v.value
                )
            })?;
        }
        Ok(v)
    }

    pub fn iterate_to_n(self, n: u64) -> Result<Point, Error> {
        let mut v = self;
        for i in v.iterations..n {
            if v.escaped {
                return Ok(v);
            }
            v.iterate().with_context(|| {
                format!(
                    "Iterate to n {} of {}, iteration {}, value {:?}",
                    i, n, v.iterations, v.value
                )
            })?;
        }
        Ok(v)
    }

    pub fn value(&self) -> &Complex {
        &self.value
    }
}

#[cfg(test)]
mod test {
    use super::Point;
    use crate::complex::{Complex, Fix2x61};
    use anyhow::Error;
    use std::convert::TryInto;

    // #[test]
    // fn two_is_escaped() {
    //     let two: Point = Point::from_parts(2i64, 0i64);
    //
    //     assert_eq!(two.escaped, true);
    //     assert_eq!(two.iterations, 0);
    //
    //     let r = two.iterate_n(1_000_000);
    //     assert_eq!(r.iterations, 0);
    // }

    #[test]
    fn zero_never_escapes() -> Result<(), Error> {
        let zero: Point = Point::ORIGIN;
        let target_count = 1_000_000;
        let r = zero.iterate_n(target_count)?;

        assert_eq!(r.escaped, false);
        assert_eq!(r.iterations, target_count);
        Ok(())
    }

    #[test]
    fn iterate_to_works() -> Result<(), Error> {
        let zero: Point = Point::ORIGIN;
        let ten = zero.iterate_n(10)?;
        let target_count = 1_000_000;
        let r = ten.iterate_to_n(target_count)?;

        assert_eq!(r.escaped, false);
        assert_eq!(r.iterations, target_count);
        Ok(())
    }

    #[test]
    fn one_escapes() -> Result<(), Error> {
        let i: Point = Point::from_parts(&Fix2x61::one(), &Fix2x61::zero());
        let target_count = 1_000_000;
        let r = i.iterate_n(target_count)?;

        assert_eq!(r.escaped, true);
        assert_eq!(r.iterations, 1);
        Ok(())
    }

    #[test]
    fn iterates_correctly() -> Result<(), Error> {
        let mut c: Point = Point::from_parts(&(-Fix2x61::one()), &(0.5).try_into()?);
        c.iterate()?;

        assert_eq!(c.escaped, false);
        assert_eq!(c.iterations, 1);
        assert_eq!(
            c.value,
            Complex::new((-0.25f64).try_into()?, (-0.5f64).try_into()?)
        );
        Ok(())
    }
}
