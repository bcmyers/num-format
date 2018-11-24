use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

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

impl_DisplayInteger!(NonZeroU8);
impl_DisplayInteger!(NonZeroU16);
impl_DisplayInteger!(NonZeroU32);
impl_DisplayInteger!(NonZeroU64);
impl_DisplayInteger!(NonZeroUsize);
impl_DisplayInteger!(NonZeroU128);
