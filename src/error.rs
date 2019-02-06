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

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

#[cfg(feature = "std")]
mod standard {
    use crate::{Error, ErrorKind};

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                C(_msg) => "received unexpected data from C.",
                Capacity(_n) => "input exceeds capacity.",
                Other(_msg) => "other miscellaneous error.",
                ParseLocale(_msg) => "failed to parse input into a Locale.",
            }
        }
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            use self::ErrorKind::*;
            match self.kind {
                C(_msg) => None,
                Capacity(_n) => None,
                Other(_msg) => None,
                ParseLocale(_msg) => None,
            }
        }
    }
}
