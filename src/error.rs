use core::fmt;

use arrayvec::ArrayString;

use crate::ErrorKind;

pub(crate) const MAX_ERR_LEN: usize = 256;

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
}

#[allow(unused)]
impl Error {
    pub(crate) fn capacity(len: usize, cap: usize) -> Error {
        Error {
            kind: ErrorKind::Capacity { len, cap },
        }
    }

    pub(crate) fn decoding<B, S>(bytes: B, encoding_label: S) -> Error
    where
        B: AsRef<[u8]>,
        S: AsRef<str>,
    {
        let _ = bytes;
        let _ = encoding_label;
        unimplemented!()
    }

    pub(crate) fn null_ptr(function_name: &str) -> Error {
        let _ = function_name;
        unimplemented!()
    }

    pub(crate) fn parse_locale<S>(msg: S) -> Error
    where
        S: AsRef<str>,
    {
        let _ = msg;
        unimplemented!()
    }

    pub(crate) fn unix<S>(msg: S) -> Error
    where
        S: AsRef<str>,
    {
        let _ = msg;
        unimplemented!()
    }

    pub(crate) fn unsupported_encoding<S>(encoding_label: S) -> Error
    where
        S: AsRef<str>,
    {
        let _ = encoding_label;
        unimplemented!()
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
