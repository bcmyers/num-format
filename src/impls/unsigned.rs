use crate::details::{Formatter, Number};
use crate::display::DisplayInteger;

macro_rules! impl_DisplayInteger {
    ($t:ident) => {
        impl DisplayInteger for $t {
            fn formatter(&self) -> Formatter<'static> {
                Number::Integer {
                    s: self.to_string(),
                    is_positive: true,
                }
                .into()
            }
        }
    };
}

impl_DisplayInteger!(u8);
impl_DisplayInteger!(u16);
impl_DisplayInteger!(u32);
impl_DisplayInteger!(usize);
impl_DisplayInteger!(u64);
impl_DisplayInteger!(u128);
