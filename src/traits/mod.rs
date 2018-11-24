mod to_formatted_str;
#[cfg(feature = "std")]
mod to_formatted_string;
#[cfg(feature = "std")]
mod write_formatted;

pub use self::sealed::Sealed;
pub use self::to_formatted_str::ToFormattedStr;
#[cfg(feature = "std")]
pub use self::to_formatted_string::ToFormattedString;
#[cfg(feature = "std")]
pub use self::write_formatted::WriteFormatted;

mod sealed {
    pub trait Sealed {}
}
