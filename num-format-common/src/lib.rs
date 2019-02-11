//! TODO

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    dead_code,
    deprecated,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_imports,
    unused_macros,
    unused_mut,
    unused_results,
    unused_parens,
    unused_unsafe,
    unused_variables
)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod constants;
mod error;
mod error_kind;
mod format;
mod grouping;
mod locale;
pub mod utils;

pub use self::error::Error;
pub use self::error_kind::ErrorKind;
pub use self::format::Format;
pub use self::grouping::Grouping;
pub use self::locale::Locale;
