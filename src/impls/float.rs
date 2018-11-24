use crate::details::{Formatter, Number};
use crate::display::DisplayFloat;

macro_rules! impl_DisplayFloat {
    ($t:ident) => {
        impl DisplayFloat for $t {
            fn formatter(&self) -> Formatter<'static> {
                Number::FloatNan.into()
            }
        }
    };
}

impl_DisplayFloat!(f32);
impl_DisplayFloat!(f64);
