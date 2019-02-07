//! Utility types needed if you want to implement [`Format`] on your own type.
//!
//! [`Format`]: trait.Format.html

mod max_len_strs;
mod pointer;
mod separator;
mod write_bytes;
#[cfg(feature = "num-bigint")]
mod write_str_to_buffer;

pub use self::max_len_strs::{InfinityStr, MinusSignStr, NanStr};
pub(crate) use self::pointer::Pointer;
pub(crate) use self::separator::Separator;
pub(crate) use self::write_bytes::{write_one_byte, write_two_bytes};
#[cfg(feature = "num-bigint")]
pub(crate) use self::write_str_to_buffer::write_str_to_buffer;
