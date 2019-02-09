use core::fmt;

use arrayvec::ArrayString;

use crate::constants::MAX_ERR_LEN;

/// This crate's error kind.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum ErrorKind {
    /// Input exceeds capacity.
    Capacity {
        /// Length of the input in bytes.
        len: usize,
        /// Capacity of the buffer in bytes.
        cap: usize,
    },

    /// Other miscellaneous error.
    Other {
        /// Additional details.
        msg: ArrayString<[u8; MAX_ERR_LEN]>,
    },

    /// Failed to parse input into a Locale.
    ParseLocale {
        /// First 256 bytes of the provided input.
        input: ArrayString<[u8; MAX_ERR_LEN]>,
    },

    /// Unix API error.
    Unix {
        /// Additional details.
        msg: ArrayString<[u8; MAX_ERR_LEN]>,
    },

    /// Windows API error.
    Windows {
        /// Additional details.
        msg: ArrayString<[u8; MAX_ERR_LEN]>,
    },
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorKind::*;
        match self {
            Capacity { len, cap } => write!(
                f,
                "attempted to write input of length {} bytes into buffer with capacity {} bytes.",
                len, cap
            ),
            Other { msg } => write!(f, "miscellaneous error: {}", msg),
            ParseLocale { input } => {
                write!(f, "failed to parse input into a Locale. input: {}", input)
            }
            Unix { msg } => write!(f, "unix api error: {}", msg),
            Windows { msg } => write!(f, "windows api error: {}", msg),
        }
    }
}
