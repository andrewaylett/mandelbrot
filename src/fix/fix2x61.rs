use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Neg, Sub};

use num::abs;

use crate::complex::{FixError, FixResult};
use crate::fix::fix4x123::Fix4x123;

#[derive(Copy, Clone, PartialEq)]
pub struct Fix2x61(pub(crate) i64);

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
