use core::fmt;

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN};
use crate::errors::Error;

/// Simple wrapper type for a `&str` to make sure its length is less than the maximum for
/// an infinity symbol (64 bytes).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InfinityStr<'a>(&'a str);

impl<'a> InfinityStr<'a> {
    /// Constructs an `InfinityStr`, ensuring that the length is less than the maximum for
    /// an infinity symbol (64 bytes).
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the provided `&str`'s length is more than 64 bytes.
    pub fn new(s: &'a str) -> Result<InfinityStr<'a>, Error> {
        if s.len() > MAX_INF_LEN {
            return Err(Error::capacity(MAX_INF_LEN));
        }
        Ok(InfinityStr(s))
    }

    /// Allows recovery of the initial / wrapped `&str`.
    pub fn into_str(self) -> &'a str {
        self.0
    }
}

impl<'a> fmt::Debug for InfinityStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<'a> fmt::Display for InfinityStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Simple wrapper type for a `&str` to make sure its length is less than the maximum for
/// a minus sign (7 bytes).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MinusSignStr<'a>(&'a str);

impl<'a> MinusSignStr<'a> {
    /// Constructs a `MinusSignStr`, ensuring that the length is less than the maximum for
    /// a minus sign (7 bytes).
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the provided `&str`'s length is more than 7 bytes.
    pub fn new(s: &'a str) -> Result<MinusSignStr<'a>, Error> {
        if s.len() > MAX_MIN_LEN {
            return Err(Error::capacity(MAX_MIN_LEN));
        }
        Ok(MinusSignStr(s))
    }

    /// Allows recovery of the initial / wrapped `&str`.
    pub fn into_str(self) -> &'a str {
        self.0
    }
}

impl<'a> fmt::Debug for MinusSignStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<'a> fmt::Display for MinusSignStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Simple wrapper type for a `&str` to make sure its length is less than the maximum for
/// a nan symbol (64 bytes).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NanStr<'a>(&'a str);

impl<'a> NanStr<'a> {
    /// Constructs an `NanStr`, ensuring that the length is less than the maximum for
    /// a nan symbol (64 bytes).
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the provided `&str`'s length is more than 64 bytes.
    pub fn new(s: &'a str) -> Result<NanStr<'a>, Error> {
        if s.len() > MAX_NAN_LEN {
            return Err(Error::capacity(MAX_NAN_LEN));
        }
        Ok(NanStr(s))
    }

    /// Allows recovery of the initial / wrapped `&str`.
    pub fn into_str(self) -> &'a str {
        self.0
    }
}

impl<'a> fmt::Debug for NanStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<'a> fmt::Display for NanStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
