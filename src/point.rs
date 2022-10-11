use anyhow::{Context, Error};

use crate::complex::{Complex, FixError};
use crate::fix::fix2x61::Fix2x61;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    loc: Complex,
    value: Complex,
    pub iterations: u64,
    pub escaped: bool,
    pub escape_candidate: bool,
}

impl Point {
    pub const ORIGIN: Point = Point::new(Complex::new(Fix2x61::zero(), Fix2x61::zero()));

    pub fn from_parts(x: &Fix2x61, y: &Fix2x61) -> Point {
        Point::new(Complex::new(*x, *y))
    }

    pub const fn new(c: Complex) -> Point {
        let escaped = false;
        let escape_candidate = false;
        Point {
            loc: c,
            value: c,
            iterations: 0,
            escaped,
            escape_candidate,
        }
    }

    // Microbenchmarks suggest no benefit from an inline attribute
    pub fn iterate(&mut self) -> Result<(), Error> {
        if !self.escaped {
            let iterated = self.value.iterate_mandelbrot(&self.loc);
            if let Err(FixError::Escaped) = iterated {
                self.escaped = true;
            } else {
                iterated.with_context(|| format!("Iterating, value after: {:?}", self.value))?
            }
            self.iterations += 1;
        }
        Ok(())
    }

    pub fn iterate_n(&mut self, n: u64) -> Result<(), Error> {
        for i in 0..n {
            if self.escaped {
                return Ok(());
            }
            self.iterate().with_context(|| {
                format!(
                    "Iterate n {} of {}, iteration {}, value {:?}",
                    i, n, self.iterations, self.value
                )
            })?;
        }
        Ok(())
    }

    pub fn iterate_to_n(&mut self, n: u64) -> Result<(), Error> {
        if !self.escape_candidate {
            return Ok(());
        }

        let start = self.iterations;
        for i in start..n {
            if self.escaped {
                return Ok(());
            }
            self.iterate().with_context(|| {
                format!(
                    "Iterate to n {} of {}, iteration {}, value {:?}",
                    i, n, self.iterations, self.value
                )
            })?;
        }
        Ok(())
    }

    pub fn value(&self) -> &Complex {
        &self.value
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use anyhow::Error;

    use crate::complex::Complex;
    use crate::fix::fix2x61::Fix2x61;

    use super::Point;

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
        let mut zero: Point = Point::ORIGIN;
        let target_count = 1_000_000;
        zero.iterate_n(target_count)?;

        assert!(!zero.escaped);
        assert_eq!(zero.iterations, target_count);
        Ok(())
    }

    #[test]
    fn iterate_to_works() -> Result<(), Error> {
        let mut zero: Point = Point::ORIGIN;
        zero.escape_candidate = true;
        zero.iterate_n(10)?;
        let target_count = 1_000_000;
        zero.iterate_to_n(target_count)?;

        assert!(!zero.escaped);
        assert_eq!(zero.iterations, target_count);
        Ok(())
    }

    #[test]
    fn one_escapes() -> Result<(), Error> {
        let mut i: Point = Point::from_parts(&Fix2x61::one(), &Fix2x61::zero());
        let target_count = 1_000_000;
        i.iterate_n(target_count)?;

        assert!(i.escaped);
        assert_eq!(i.iterations, 1);
        Ok(())
    }

    #[test]
    fn iterates_correctly() -> Result<(), Error> {
        let mut c: Point = Point::from_parts(&(-Fix2x61::one()), &(0.5).try_into()?);
        c.iterate()?;

        assert!(!c.escaped);
        assert_eq!(c.iterations, 1);
        assert_eq!(
            c.value,
            Complex::new((-0.25f64).try_into()?, (-0.5f64).try_into()?)
        );
        Ok(())
    }
}
