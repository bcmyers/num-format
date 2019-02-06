use arrayvec::ArrayString;
use failure::Fail;

use crate::constants::MAX_ERR_LEN;

/// This crate's error kind.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Fail)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum ErrorKind {
    /// Received unexpected data from C (e.g. a NULL pointer).
    ///
    /// Associated data is the first 256 bytes of a custom error message.
    #[fail(display = "received unexpected data from C; {}", _0)]
    C(ArrayString<[u8; MAX_ERR_LEN]>),

    /// Input exceeds capacity.
    ///
    /// Associated data is the maximum length of the input in bytes.
    #[fail(display = "input exceeds capacity of {}", _0)]
    Capacity(usize),

    /// Other miscellaneous error.
    ///
    /// Associated data is the first 256 bytes of a custom error message.
    #[fail(display = "{}", _0)]
    Other(ArrayString<[u8; MAX_ERR_LEN]>),

    /// Failed to parse input into a Locale.
    ///
    /// Associated data is the first 256 bytes of the provided input.
    #[fail(display = "failed to parse {} into a Locale", _0)]
    ParseLocale(ArrayString<[u8; MAX_ERR_LEN]>),
}
