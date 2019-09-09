use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    r: f64,
    i: f64,
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Complex {
        Complex { r, i }
    }

    pub fn escaped(&self) -> bool {
        self.r * self.r + self.i * self.i >= 4.0
    }

    pub fn i() -> Complex {
        Complex::new(0.0, 1.0)
    }
}

impl From<u32> for Complex {
    fn from(r: u32) -> Complex {
        Complex::new(r as f64, 0.0)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;

    #[test]
    fn sq_i() {
        assert_eq!(Complex::i() * Complex::i(), Complex::new(-1.0, 0.0));
    }

    #[test]
    fn one_plus_one() {
        let one = Complex::from(1);
        let two = Complex::from(2);
        assert_eq!(one + one, two)
    }

    #[test]
    fn three_i() {
        let three = Complex::from(3);
        let i = Complex::i();
        assert_eq!(three * i, Complex::new(0.0, 3.0))
    }
}
