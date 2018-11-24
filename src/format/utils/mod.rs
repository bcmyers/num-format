//! Utility types needed if you want to implement [`Format`] on your own type.
//!
//! [`Format`]: trait.Format.html

mod safe_strs;

pub use self::safe_strs::{InfinityStr, MinusSignStr, NanStr};
