use num_traits::{Bounded, PrimInt};
use std::ops::Not;

trait Bigger: PrimInt {
    type Larger: PrimInt;
}

macro_rules! bigger {
    ($s:ty, $l:ty) => {
        impl Bigger for $s {
            type Larger = $l;
        }
    };
}

bigger!(u8, u16);
bigger!(u16, u32);
bigger!(u32, u64);
bigger!(u64, u128);
bigger!(i8, i16);
bigger!(i16, i32);
bigger!(i32, i64);
bigger!(i64, i128);

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Extending<T: Bigger>(T);

impl<T: Bigger> From<T> for Extending<T> {
    fn from(f: T) -> Self {
        Extending(f)
    }
}

impl<T: Bigger> Bounded for Extending<T> {
    fn min_value() -> Self {
        T::min_value().into()
    }

    fn max_value() -> Self {
        T::max_value().into()
    }
}

impl<T: Bigger> Not for Extending<T> {
    type Output = Extending<<T as std::ops::Not>::Output>;

    fn not(self) -> Self::Output {
        self.0.not().into()
    }
}
