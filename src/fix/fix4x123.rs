use std::ops::{Add, Sub};

use crate::complex::{FixError, FixResult};
use crate::fix::fix2x61::Fix2x61;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Fix4x123(pub(crate) i128);

impl Fix4x123 {
    // One sign bit, four int bits, 123 mantissa bits

    pub const ZERO: Self = Fix4x123(0);

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
            Err(FixError::Overflow { op: "truncate" })
        }
    }
}

impl Default for Fix4x123 {
    fn default() -> Self {
        Fix4x123::ZERO
    }
}

impl Add for Fix4x123 {
    type Output = FixResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if let Some(res) = self.0.checked_add(rhs.0) {
            Ok(Self(res))
        } else {
            Err(FixError::Overflow {
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
            Err(FixError::Overflow {
                op: "Fix4x123::sub",
            })
        }
    }
}

impl From<Fix2x61> for Fix4x123 {
    fn from(val: Fix2x61) -> Self {
        Fix4x123((val.0 as i128) << 62)
    }
}
