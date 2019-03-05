use crate::buffer::Buffer;
use crate::error::Error;
use crate::format::Format;
use crate::sealed::Sealed;

/// Marker trait for number types that can be formatted without heap allocation (see [`Buffer`]).
///
/// This trait is sealed; so you may not implement it on your own types.
///
/// [`Buffer`]: struct.Buffer.html
pub trait ToFormattedStr: Sealed + Sized {
    #[doc(hidden)]
    fn read_to_buffer<F>(&self, buf: &mut Buffer, format: &F) -> usize
    where
        F: Format;

    #[doc(hidden)]
    fn from_formatted_str<F>(s: &str, format: &F) -> Result<Self, Error>
    where
        F: Format;
}
