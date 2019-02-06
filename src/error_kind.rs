use core::fmt;

use arrayvec::ArrayString;

use crate::constants::MAX_ERR_LEN;

/// This crate's error kind.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum ErrorKind {
    /// Received unexpected data from C (e.g. a NULL pointer).
    ///
    /// Associated data is the first 256 bytes of a custom error message.
    C(ArrayString<[u8; MAX_ERR_LEN]>),

    /// Input exceeds capacity.
    ///
    /// Associated data is the maximum length of the input in bytes.
    Capacity(usize),

    /// Other miscellaneous error.
    ///
    /// Associated data is the first 256 bytes of a custom error message.
    Other(ArrayString<[u8; MAX_ERR_LEN]>),

    /// Failed to parse input into a Locale.
    ///
    /// Associated data is the first 256 bytes of the provided input.
    ParseLocale(ArrayString<[u8; MAX_ERR_LEN]>),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorKind::*;
        match self {
            C(msg) => write!(f, "received unexpected data from C. {}", msg),
            Capacity(n) => write!(f, "input exceeds capacity of {} bytes.", n),
            Other(msg) => write!(f, "other miscellaneous error. {}", msg),
            ParseLocale(msg) => write!(f, "failed to parse input into a Locale. {}", msg),
        }
    }
}
