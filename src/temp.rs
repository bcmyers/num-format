use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

pub enum Number {
    Unsigned(u128),
    Signed(i128),
    Float(f64),
    Other { s: String, is_positive: bool },
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::Unsigned(value as u128)
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::Unsigned(value as u128)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::Unsigned(value as u128)
    }
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Number::Unsigned(value as u128)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::Unsigned(value as u128)
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Number::Unsigned(value)
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::Signed(value as i128)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::Signed(value as i128)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Signed(value as i128)
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        Number::Signed(value as i128)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::Signed(value as i128)
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::Signed(value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(value as f64)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

impl From<NonZeroU8> for Number {
    fn from(value: NonZeroU8) -> Self {
        Number::Unsigned(value.get() as u128)
    }
}

impl From<NonZeroU16> for Number {
    fn from(value: NonZeroU16) -> Self {
        Number::Unsigned(value.get() as u128)
    }
}

impl From<NonZeroU32> for Number {
    fn from(value: NonZeroU32) -> Self {
        Number::Unsigned(value.get() as u128)
    }
}

impl From<NonZeroUsize> for Number {
    fn from(value: NonZeroUsize) -> Self {
        Number::Unsigned(value.get() as u128)
    }
}

impl From<NonZeroU64> for Number {
    fn from(value: NonZeroU64) -> Self {
        Number::Unsigned(value.get() as u128)
    }
}

impl From<NonZeroU128> for Number {
    fn from(value: NonZeroU128) -> Self {
        Number::Unsigned(value.get())
    }
}
