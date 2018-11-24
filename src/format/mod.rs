//! Module containing format options (i.e. how you would like your number to look).
//! Includes the [`Format`] trait and three concrete types that implement it: [`CustomFormat`],
//! [`Environment`], and [`Locale`].
//!
//! [`CustomFormat`]: format/struct.CustomFormat.html
//! [`Environment`]: format/struct.Environment.html
//! [`Format`]: format/trait.Format.html
//! [`Locale`]: format/enum.Locale.html

#![allow(clippy::module_inception)]

mod custom_format;
mod custom_format_builder;
mod environment;
mod format;
mod grouping;
mod locale;
pub mod utils;

pub use self::custom_format::CustomFormat;
pub use self::custom_format_builder::CustomFormatBuilder;
pub use self::environment::Environment;
pub use self::format::Format;
pub use self::grouping::Grouping;
pub use self::locale::Locale;
