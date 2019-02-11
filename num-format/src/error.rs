use core::fmt;

use arrayvec::ArrayString;

use crate::constants::MAX_ERR_LEN;
use crate::ErrorKind;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// This crate's error type.
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Constructs a new [`Error`] with kind [`ErrorKind::Other`].
    ///
    /// [`Error`]: struct.Error.html
    /// [`ErrorKind::Other`]: enum.ErrorKind.html#variant.Other
    pub fn new<S>(msg: S) -> Error
    where
        S: AsRef<str>,
    {
        let s = msg.as_ref();
        let s = if s.len() > MAX_ERR_LEN {
            &s[0..MAX_ERR_LEN]
        } else {
            s
        };
        Error {
            kind: ErrorKind::Other {
                msg: ArrayString::from(s).unwrap(),
            },
        }
    }

    /// Returns the [`ErrorKind`].
    ///
    /// [`ErrorKind`]: enum.ErrorKind.html
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub(crate) fn capacity(len: usize, cap: usize) -> Error {
        Error {
            kind: ErrorKind::Capacity { len, cap },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

#[cfg(feature = "std")]
mod standard {
    use crate::{Error, ErrorKind};

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            use self::ErrorKind::*;
            match self.kind {
                Capacity { .. } => None,
                Other { .. } => None,
            }
        }
    }
}
