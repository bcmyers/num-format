//! Utility crate for programmatically generating a rust module / enum from CLDR json files.

#![deny(dead_code)]
#![deny(deprecated)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_macros)]
#![deny(unused_mut)]
#![deny(unused_results)]
#![deny(unused_parens)]
#![deny(unused_unsafe)]
#![deny(unused_variables)]
#![recursion_limit = "256"]

mod create_module;
mod parse_data;
#[cfg(feature = "nightly")]
mod rustfmt;
mod utils;

pub use self::create_module::create_module;
pub use self::parse_data::parse_data;
#[cfg(feature = "nightly")]
pub use self::rustfmt::rustfmt;
