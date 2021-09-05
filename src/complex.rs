use num::abs;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Neg, Sub};
use thiserror::Error;

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Fix4x123(i128);
#[derive(Copy, Clone, PartialEq)]
pub struct Fix2x61(i64);

impl Fix2x61 {
    const fn try_from_i8(value: i8) -> Result<Fix2x61, FixError> {
        if value >= 4 || value <= -4 {
            Err(FixError::OverFlow {
                op: "Fix2x61::try_from(i8)",
            })
        } else {
            Ok(Fix2x61((value as i64) << 61))
        }
    }
}

impl Debug for Fix2x61 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display_value = (self.0 as f64) / ((1i64 << 61) as f64);
        f.debug_tuple("Fix2x61").field(&display_value).finish()
    }
}

#[derive(Clone, Debug, Error)]
pub enum FixError {
    #[error("Operation {op} would overflow")]
    OverFlow { op: &'static str },
    #[error("Iteration triggered escape")]
    Escaped,
}

pub type FixResult<T> = Result<T, FixError>;

impl Fix4x123 {
    // One sign bit, four int bits, 123 mantissa bits

    pub const fn zero() -> Self {
        Fix4x123(0)
    }

    pub const fn one() -> Self {
        Fix4x123(1 << 123)
    }

    pub const fn two() -> Self {
        Fix4x123(1 << 124)
    }

    pub const fn four() -> Self {
        Fix4x123(1 << 125)
    }

    pub const fn truncate(&self) -> FixResult<Fix2x61> {
        if self.0 < Fix4x123::four().0 && self.0 > -(Fix4x123::four().0) {
            Ok(Fix2x61((self.0 >> 62) as i64))
        } else {
            Err(FixError::OverFlow { op: "truncate" })
        }
    }
}

impl Default for Fix4x123 {
    fn default() -> Self {
        Fix4x123::zero()
    }
}

impl Default for Fix2x61 {
    fn default() -> Self {
        Fix2x61::zero()
    }
}

impl Neg for Fix2x61 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fix2x61(-self.0)
    }
}

impl Fix2x61 {
    // One sign bit, two int bits, 61 mantissa bits

    pub const fn zero() -> Self {
        Fix2x61(0)
    }

    pub const fn one() -> Self {
        Fix2x61(1 << 61)
    }

    pub const fn two() -> Self {
        Fix2x61(1 << 62)
    }

    pub const fn power_of_two(pow: i8) -> FixResult<Self> {
        if pow > 2 || pow < -61 {
            Err(FixError::OverFlow { op: "power_of_two" })
        } else {
            Ok(Fix2x61(1 << (61 + pow)))
        }
    }
}

impl Add for Fix4x123 {
    type Output = FixResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if let Some(res) = self.0.checked_add(rhs.0) {
            Ok(Self(res))
        } else {
            Err(FixError::OverFlow {
                op: "Fix4x123::add",
            })
        }
    }
}

impl Sub for Fix4x123 {
    type Output = FixResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Some(res) = self.0.checked_sub(rhs.0) {
            Ok(Self(res))
        } else {
            Err(FixError::OverFlow {
                op: "Fix4x123::sub",
            })
        }
    }
}

impl Add for Fix2x61 {
    type Output = FixResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if let Some(res) = self.0.checked_add(rhs.0) {
            Ok(Self(res))
        } else {
            Err(FixError::OverFlow { op: "Fix2x61::add" })
        }
    }
}

impl Sub for Fix2x61 {
    type Output = FixResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Some(res) = self.0.checked_sub(rhs.0) {
            Ok(Self(res))
        } else {
            Err(FixError::OverFlow { op: "Fix2x61::sub" })
        }
    }
}

impl Mul for Fix2x61 {
    type Output = Fix4x123;

    fn mul(self, rhs: Self) -> Self::Output {
        Fix4x123((self.0 as i128 * rhs.0 as i128) << 1)
    }
}

impl TryFrom<i8> for Fix2x61 {
    type Error = FixError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Fix2x61::try_from_i8(value)
    }
}

impl TryFrom<f64> for Fix2x61 {
    type Error = FixError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if abs(value) >= 4.0 {
            Err(FixError::OverFlow {
                op: "Fix2x61::try_from(f64)",
            })
        } else {
            Ok(Fix2x61((value * (1i64 << 61) as f64) as i64))
        }
    }
}

impl From<Fix2x61> for Fix4x123 {
    fn from(val: Fix2x61) -> Self {
        Fix4x123((val.0 as i128) << 62)
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Complex {
    pub r: Fix2x61,
    pub i: Fix2x61,
}

const fn overflow_escapes(e: FixError) -> FixError {
    if let FixError::OverFlow { op: _ } = e {
        FixError::Escaped
    } else {
        e
    }
}

impl Complex {
    pub const fn new(r: Fix2x61, i: Fix2x61) -> Complex {
        Complex { r, i }
    }

    pub const fn i() -> Complex {
        Complex::new(Fix2x61::zero(), Fix2x61::one())
    }

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
    use super::Complex;
    use crate::complex::{Fix2x61, Fix4x123, FixResult};
    use std::convert::{TryFrom, TryInto};

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
