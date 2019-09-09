use num::traits::{One, Zero};
use std::ops::{Add, Mul, Neg, MulAssign, AddAssign};
use num_traits::{NumAssignRef};

pub trait Part: NumAssignRef + Copy + PartialOrd {}

impl<T> Part for T where T: NumAssignRef + Copy + PartialOrd {}

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Complex<Unit: Part>
{
    r: Unit,
    i: Unit,
}


impl<Unit: Part> Complex<Unit> {
    pub fn new(r: Unit, i: Unit) -> Complex<Unit> {
        Complex { r, i }
    }

    #[inline]
    pub fn escaped(&self) -> bool {
        let r = self.r.clone();
        let i = self.i.clone();
        let left = r.clone() * r;
        let right = i.clone() * i;
        left + right >= (<Unit as One>::one() + One::one() + One::one() + One::one())
    }

    pub fn i() -> Complex<Unit> {
        Complex::new(Zero::zero(), One::one())
    }
}

impl<Unit: Part> Complex<Unit> {
    pub fn from_parts<T: Into<Unit>>(r: T, i: T) -> Complex<Unit> {
        Complex::new(r.into(), i.into())
    }
}

impl<Unit: Part> One for Complex<Unit> {
    fn one() -> Self {
        Complex::new(One::one(), Zero::zero())
    }
}

impl<Unit: Part> Zero for Complex<Unit> {
    fn zero() -> Self {
        Complex::new(Zero::zero(), Zero::zero())
    }

    fn is_zero(&self) -> bool {
        self.r == Zero::zero() && self.i == Zero::zero()
    }
}

impl<'a, 'b, Unit: Part> Mul<Complex<&'b Unit>> for &'a Complex<Unit>
where for<'f> &'f Unit: Part,
{
    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: Complex<&'b Unit>) -> Complex<Unit> {
        Complex {
            r: *&self.r * *rhs.r - *&self.i * *rhs.i,
            i: *&self.r * *rhs.i + *&self.i * *rhs.r,
        }
    }
}

impl<'a, Unit: Part> Mul<Complex<Unit>> for &'a Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: *&self.r * rhs.r - *&self.i * rhs.i,
            i: *&self.r * rhs.i + *&self.i * rhs.r,
        }
    }
}

impl<'b, Unit: Part> Mul<&'b Complex<Unit>> for Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: &Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl<Unit: Part> Mul for Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl<'a, 'b, Unit: Part> Add<&'b Complex<Unit>> for &'a Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: &Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl<'a, Unit: Part> Add<Complex<Unit>> for &'a Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl<'b, Unit: Part> Add<&'b Complex<Unit>> for Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: &Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl<Unit: Part> Add<Complex<Unit>> for Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl<Unit: Part> Neg for Complex<Unit> {
    type Output = Complex<Unit>;

    fn neg(self) -> Self::Output {
        Complex::new(<Unit as Zero>::zero() - self.r, <Unit as Zero>::zero() - self.i)
    }
}

impl<Unit: Part> MulAssign for Complex<Unit> {
    fn mul_assign(&mut self, rhs: Self) {
        let r = self.r;
        let i = self.i;
        self.r = r * rhs.r - i * rhs.i;
        self.i = r * rhs.i + i * rhs.r;
    }
}

impl<Unit: Part> AddAssign for Complex<Unit> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;
    use num::One;

    #[test]
    fn sq_i() {
        let i: Complex<i64> = Complex::i();
        let minus_one: Complex<i64> = -<Complex<i64> as One>::one();
        assert_eq!(i * i, minus_one);
    }

    #[test]
    fn one_plus_one() {
        let one: Complex<i64> = Complex::from_parts(1, 0);
        let two: Complex<i64> = Complex::from_parts(2, 0);
        assert_eq!(one + one, two)
    }

    #[test]
    fn three_i() {
        let three: Complex<i64> = Complex::from_parts(3, 0);
        let i: Complex<i64> = Complex::i();
        assert_eq!(three * i, Complex::from_parts(0, 3))
    }
}
