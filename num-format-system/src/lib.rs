//! TODO

#![cfg(any(unix, windows))]
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
#![doc(html_root_url = "https://docs.rs/num-format-system/0.3.0")]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod error;
mod system_locale;

pub use self::error::Error;
pub use self::system_locale::SystemLocale;
