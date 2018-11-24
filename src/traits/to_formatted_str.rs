use crate::buffer::Buffer;
use crate::format::Format;
use crate::traits::Sealed;

/// Marker trait for number types that can be formatted without heap allocation (see [`Buffer`]).
///
/// This trait is Sealed; so you may not implement it on your own types.
///
/// [`Buffer`]: struct.Buffer.html
pub trait ToFormattedStr: Sealed {
    #[doc(hidden)]
    fn read_to_buffer<F: Format>(&self, buf: &mut Buffer, format: &F) -> usize;
}
