//! Utility crate for programmatically generating a rust module / enum from CLDR json files.

#![deny(
    dead_code,
    deprecated,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused
)]
#![recursion_limit = "256"]

mod create_module;
mod parse_data;
mod utils;

pub use self::create_module::create_module;
pub use self::parse_data::parse_data;
