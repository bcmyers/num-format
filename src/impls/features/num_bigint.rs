use num_bigint::{BigInt, BigUint, Sign};

use crate::details::{Formatter, Number};
use crate::display::DisplayInteger;

impl DisplayInteger for BigInt {
    fn formatter(&self) -> Formatter<'static> {
        Number::Integer {
            s: self.to_string(),
            is_positive: match self.sign() {
                Sign::Minus => false,
                Sign::NoSign => true,
                Sign::Plus => true,
            },
        }
        .into()
    }
}

impl DisplayInteger for BigUint {
    fn formatter(&self) -> Formatter<'static> {
        Number::Integer {
            s: self.to_string(),
            is_positive: true,
        }
        .into()
    }
}
