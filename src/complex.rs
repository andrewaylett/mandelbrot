use std::convert::TryInto;
use std::str::FromStr;

use anyhow::bail;
use num::abs;
use thiserror::Error;

use crate::fix::fix2x61::Fix2x61;
use crate::fix::fix4x123::Fix4x123;

#[derive(Clone, Debug, Error)]
pub enum FixError {
    #[error("Operation {op} would overflow")]
    Overflow { op: &'static str },
    #[error("Operation {op} would underflow")]
    Underflow { op: &'static str },
    #[error("Iteration triggered escape")]
    Escaped,
}

pub type FixResult<T> = Result<T, FixError>;

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Complex {
    pub r: Fix2x61,
    pub i: Fix2x61,
}

impl Complex {
    pub const fn zero() -> Complex {
        Complex {
            r: Fix2x61::zero(),
            i: Fix2x61::zero(),
        }
    }
}

impl FromStr for Complex {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(',').collect();
        if split.len() != 2 {
            bail!("Must supply a complex in the form '0.0,0.0'")
        }

        let r = f64::from_str(split[0])?;
        let i = f64::from_str(split[1])?;

        Ok(Complex::new(r.try_into()?, i.try_into()?))
    }
}

const fn overflow_escapes(e: FixError) -> FixError {
    if let FixError::Overflow { op: _ } = e {
        FixError::Escaped
    } else {
        e
    }
}

impl Complex {
    pub const fn new(r: Fix2x61, i: Fix2x61) -> Complex {
        Complex { r, i }
    }

    #[inline(always)] // Microbenchmarks suggest inlining slows down single iterations but speeds up full renders
    pub fn iterate_mandelbrot(&self, loc: &Complex) -> FixResult<Complex> {
        let r = self.r;
        let i = self.i;

        // Square
        let (r, i) = ((r * r - i * i)?, (r * i + r * i)?);

        // Add
        let (r, i) = ((r + From::from(loc.r))?, (i + From::from(loc.i))?);

        // Truncate
        let (r, i) = (
            r.truncate().map_err(overflow_escapes)?,
            i.truncate().map_err(overflow_escapes)?,
        );

        // Escape check
        if abs((r * r + i * i).map_err(overflow_escapes)?.0) >= Fix4x123::four().0 {
            Err(FixError::Escaped)
        } else {
            Ok(Complex::new(r, i))
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Complex::new(Default::default(), Default::default())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::complex::FixResult;
    use crate::fix::fix2x61::Fix2x61;
    use crate::fix::fix4x123::Fix4x123;

    use super::Complex;

    #[test]
    fn add_fix_64() -> FixResult<()> {
        let one = Fix2x61::one();
        let two = one + one;
        assert_eq!(two?, Fix2x61::try_from(2)?);
        Ok(())
    }

    #[test]
    fn trunc_long() -> FixResult<()> {
        assert_eq!(Fix4x123::one().truncate()?, Fix2x61::one());
        Ok(())
    }

    #[test]
    fn mult_fix_point() {
        let one = Fix2x61::one();
        let long_one = one * one;
        assert_eq!(long_one, Fix4x123::one());
    }

    #[test]
    fn four() -> FixResult<()> {
        let one = Fix2x61::one();
        let two = (one + one)?;
        assert_eq!(two * two, Fix4x123::four());
        Ok(())
    }

    #[test]
    fn power_of_two_zero() -> FixResult<()> {
        let one = Fix2x61::one();
        let two_zero = Fix2x61::power_of_two(0)?;
        assert_eq!(one, two_zero);
        Ok(())
    }

    #[test]
    fn power_of_two_one() -> FixResult<()> {
        let two = Fix2x61::two();
        let two_one = Fix2x61::power_of_two(1)?;
        assert_eq!(two, two_one);
        Ok(())
    }

    #[test]
    fn power_of_two_minus_one() -> FixResult<()> {
        let two = Fix2x61::two();
        let two_minus_one = Fix2x61::power_of_two(-1)?;
        assert_eq!(two * two_minus_one, Fix4x123::one());
        Ok(())
    }

    #[test]
    fn extend_one() {
        let one = Fix2x61::one();
        let extended: Fix4x123 = From::from(one);
        assert_eq!(extended, Fix4x123::one());
    }

    #[test]
    fn extend_two() -> FixResult<()> {
        let one = Fix2x61::one();
        let two = (one + one)?;
        let extended: Fix4x123 = From::from(two);
        let long_one = Fix4x123::one();
        let long_two = (long_one + long_one)?;
        assert_eq!(extended, long_two);
        Ok(())
    }

    #[test]
    fn iterate_zero() -> FixResult<()> {
        let origin: Complex = Default::default();
        let iterated = origin.iterate_mandelbrot(&origin)?;
        assert_eq!(iterated, origin);
        Ok(())
    }

    #[test]
    fn iterate_i() -> FixResult<()> {
        let i = Complex::new(Fix2x61::zero(), Fix2x61::one());
        let iterated = i.iterate_mandelbrot(&i)?;
        assert_eq!(iterated, Complex::new(Fix2x61(-1 << 61), Fix2x61::one()));
        Ok(())
    }

    #[test]
    fn float_into_1() -> FixResult<()> {
        let one: Fix2x61 = 1.0.try_into()?;
        assert_eq!(one, Fix2x61::one());
        Ok(())
    }

    #[test]
    fn float_into_minus_1() -> FixResult<()> {
        let result: Fix2x61 = (-1.0).try_into()?;
        let minus_one = Fix2x61(-(Fix2x61::one().0));
        assert_eq!(result, minus_one);
        Ok(())
    }

    // #[test]
    // fn sq_i() {
    //     let i: Complex = Complex::i();
    //     let minus_one: Complex = -Complex::one();
    //     assert_eq!(i * i, minus_one);
    // }
    //
    // #[test]
    // fn one_plus_one() -> FixResult<()> {
    //     let one: Complex = Complex::from_parts(1, 0)?;
    //     let two: Complex = Complex::from_parts(2, 0)?;
    //     assert_eq!(one + one, two);
    //     Ok(())
    // }
    //
    // #[test]
    // fn three_i() {
    //     let three: Complex = Complex::from_parts(3, 0);
    //     let i: Complex = Complex::i();
    //     assert_eq!(three * i, Complex::from_parts(0, 3))
    // }
}
