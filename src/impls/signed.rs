use crate::details::{Formatter, Number};
use crate::display::DisplayInteger;

macro_rules! impl_DisplayInteger {
    ($t:ident) => {
        impl DisplayInteger for $t {
            fn formatter(&self) -> Formatter<'static> {
                Number::Integer {
                    s: self.to_string(),
                    is_positive: *self >= 0,
                }
                .into()
            }
        }
    };
}

impl_DisplayInteger!(i8);
impl_DisplayInteger!(i16);
impl_DisplayInteger!(i32);
impl_DisplayInteger!(isize);
impl_DisplayInteger!(i64);
impl_DisplayInteger!(i128);
