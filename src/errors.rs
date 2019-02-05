//! Module containing this crate's error types.

use core::fmt;

use arrayvec::ArrayString;
use failure::Fail;

use crate::constants::MAX_ERR_LEN;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Fail)]
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
            kind: ErrorKind::Other(ArrayString::from(s).unwrap()),
        }
    }

    /// Returns the [`ErrorKind`].
    ///
    /// [`ErrorKind`]: enum.ErrorKind.html
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[cfg_attr(not(feature = "std"), allow(dead_code))]
    pub(crate) fn c<S>(msg: S) -> Error
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
            kind: ErrorKind::C(ArrayString::from(s).unwrap()),
        }
    }

    pub(crate) fn capacity(cap: usize) -> Error {
        Error {
            kind: ErrorKind::Capacity(cap),
        }
    }

    pub(crate) fn parse_locale<S>(s: S) -> Error
    where
        S: AsRef<str>,
    {
        let s = s.as_ref();
        let s = if s.len() > MAX_ERR_LEN {
            &s[0..MAX_ERR_LEN]
        } else {
            s
        };
        Error {
            kind: ErrorKind::ParseLocale(ArrayString::from(s).unwrap()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

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

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}
