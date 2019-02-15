use core::fmt;

use arrayvec::ArrayString;

use crate::error::MAX_ERR_LEN;

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
        }
    }
}
