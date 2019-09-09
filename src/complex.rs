use std::ops::{Mul, Add, Neg, Sub};
use num::traits::{Zero,One};

#[derive(Debug, Clone, PartialEq)]
pub struct Complex<Unit: One + Zero>
    where Unit: Add<&Unit> + Mul<&Unit> + From<i64>,
          <Unit as Mul<&Unit>>::Output: Add<<Unit as Mul<&Unit>>::Output>,
          <<Unit as std::ops::Mul<&Unit>>::Output as std::ops::Add<<Unit as std::ops::Mul<&Unit>>::Output>>::Output: PartialOrd<Unit>
{r: Unit, i: Unit}

impl<'a, Unit: 'a + Zero + One + Clone> Complex<Unit>{

    pub fn new(r:Unit, i:Unit) -> Complex<Unit> {
        Complex {r, i}
    }

    #[inline]
    pub fn escaped(&self) -> bool {
        let r = self.r.clone();
        let i = self.i.clone();
        let left = r * &r;
        let right = i * &i;
        left + right >= From::from(4)
    }

    pub fn i() -> Complex<Unit> {
        Complex::new(Zero::zero(), One::one())
    }

    pub fn from_integers(r:i64, i:i64) -> Complex<Unit> {
        Complex::new(From::from(r), From::from(i))
    }
}

impl<Unit: 'static + Zero + One + Clone> One for Complex<Unit>
    where Unit: Add<&'static Unit> + Mul<&'static Unit> + From<i64> {

    fn one() -> Self {
        Complex::new(One::one(), Zero::zero())
    }
}

impl<Unit: Zero + One + Clone> Zero for Complex<Unit> {
    fn zero() -> Self {
        Complex::new(Zero::zero(), Zero::zero())
    }

    fn is_zero(&self) -> bool {
        self.r == Zero::zero() && self.i == Zero::zero()
    }
}

impl<Unit: Zero + One> From<i64> for Complex<Unit> {
    fn from(r: i64) -> Complex<Unit> {
        One::one() * r
    }
}

impl<'a,'b, Unit: Zero + One> Mul<&'b Complex<Unit>> for &'a Complex<Unit>
    where Unit: Add<&'b Unit> + Mul<&'b Unit> + Sub<&'b Unit> {

    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: &Complex<Unit>) -> Complex<Unit> {
        let r = *self.r;
        let i = *self.i;
        Complex {
            r: r * &rhs.r - i * &rhs.i,
            i: r * &rhs.i + i * &rhs.r
        }
    }
}

impl<'a, Unit: Zero + One> Mul<Complex<Unit>> for &'a Complex<Unit>
    where Unit: Add<Unit> + Mul<Unit> + Sub<Unit>  {

    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: &self.r * &rhs.r - &self.i * &rhs.i,
            i: &self.r * rhs.i + &self.i * rhs.r
        }
    }
}

impl<'b, Unit: Zero + One> Mul<&'b Complex<Unit>> for Complex<Unit>
    where Unit: Add<&'b Unit> + Mul<&'b Unit> + Sub<&'b Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: &Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: &self.r * &rhs.r - &self.i * &rhs.i,
            i: self.r * &rhs.i + self.i * &rhs.r
        }
    }
}

impl<Unit: Zero + One> Mul for Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn mul(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: &self.r * &rhs.r - &self.i * &rhs.i,
            i: self.r * rhs.i + self.i * rhs.r
        }
    }
}

impl<'a,'b, Unit: Zero + One> Add<&'b Complex<Unit>> for &'a Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: &Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: &self.r + &rhs.r,
            i: &self.i + &rhs.i
        }
    }
}

impl<'a, Unit: Zero + One> Add<Complex<Unit>> for &'a Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: &self.r + rhs.r,
            i: &self.i + rhs.i
        }
    }
}

impl<'b, Unit: Zero + One> Add<&'b Complex<Unit>> for Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: &Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r + &rhs.r,
            i: self.i + &rhs.i
        }
    }
}

impl<Unit: Zero + One> Add<Complex<Unit>> for Complex<Unit> {
    type Output = Complex<Unit>;

    #[inline]
    fn add(self, rhs: Complex<Unit>) -> Complex<Unit> {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl<Unit: Zero + One> Neg for Complex<Unit> {
    type Output = Complex<Unit>;

    fn neg(self) -> Self::Output {
        Zero::zero() - self
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;
    use num::One;

    #[test]
    fn sq_i() {
        let i:Complex<i64> = Complex::i();
        let minus_one:Complex<i64> = -One::one();
        assert_eq!(&i * &i, minus_one);
    }

    #[test]
    fn one_plus_one() {
        let one:Complex<i64> = From::from(1i64);
        let two:Complex<i64> = From::from(2i64);
        assert_eq!(&one + &one, two)
    }

    #[test]
    fn three_i() {
        let three:Complex<i64> = From::from(3i64);
        let i:Complex<i64> = Complex::i();
        assert_eq!(&three * &i, Complex::from_integers(0, 3))
    }
}


