//! Utility types needed if you want to implement [`Format`] on your own type.
//!
//! [`Format`]: trait.Format.html

use core::fmt;

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN, MAX_DEC_LEN, MAX_SEP_LEN};
use crate::Error;

/// Simple wrapper type for a `&str` to make sure its length is less than the maximum for
/// a decimal (8 bytes).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DecimalStr<'a>(&'a str);

impl<'a> DecimalStr<'a> {
    /// Constructs an [`DecimalStr`], ensuring that the length is less than the maximum for
    /// a decimal (8 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `&str`'s length is more than 8 bytes.
    ///
    /// [`DecimalStr`]: struct.DecimalStr.html
    pub fn new(s: &'a str) -> Result<DecimalStr<'a>, Error> {
        let len = s.len();
        if len > MAX_DEC_LEN {
            return Err(Error::capacity(len, MAX_DEC_LEN));
        }
        Ok(DecimalStr(s))
    }

    /// Allows recovery of the initial / wrapped `&str`.
    pub fn into_str(self) -> &'a str {
        self.0
    }
}

impl<'a> fmt::Debug for DecimalStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<'a> fmt::Display for DecimalStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Simple wrapper type for a `&str` to make sure its length is less than the maximum for
/// an infinity symbol (128 bytes).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InfinityStr<'a>(&'a str);

impl<'a> InfinityStr<'a> {
    /// Constructs an [`InfinityStr`], ensuring that the length is less than the maximum for
    /// an infinity symbol (128 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `&str`'s length is more than 128 bytes.
    ///
    /// [`InfinityStr`]: struct.InfinityStr.html
    pub fn new(s: &'a str) -> Result<InfinityStr<'a>, Error> {
        let len = s.len();
        if len > MAX_INF_LEN {
            return Err(Error::capacity(len, MAX_INF_LEN));
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
    /// Constructs a [`MinusSignStr`], ensuring that the length is less than the maximum for
    /// a minus sign (7 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `&str`'s length is more than 7 bytes.
    ///
    /// [`MinusSignStr`]: struct.MinusSignStr.html
    pub fn new(s: &'a str) -> Result<MinusSignStr<'a>, Error> {
        let len = s.len();
        if len > MAX_MIN_LEN {
            return Err(Error::capacity(len, MAX_MIN_LEN));
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
    /// Constructs an [`NanStr`], ensuring that the length is less than the maximum for
    /// a nan symbol (64 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `&str`'s length is more than 64 bytes.
    ///
    /// [`NanStr`]: struct.NanStr.html
    pub fn new(s: &'a str) -> Result<NanStr<'a>, Error> {
        let len = s.len();
        if len > MAX_NAN_LEN {
            return Err(Error::capacity(len, MAX_NAN_LEN));
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

/// Simple wrapper type for a `&str` to make sure its length is less than the maximum for
/// a separator (8 bytes).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SeparatorStr<'a>(&'a str);

impl<'a> SeparatorStr<'a> {
    /// Constructs an [`SeparatorStr`], ensuring that the length is less than the maximum for
    /// a separator (8 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `&str`'s length is more than 8 bytes.
    ///
    /// [`SeparatorStr`]: struct.SeparatorStr.html
    pub fn new(s: &'a str) -> Result<SeparatorStr<'a>, Error> {
        let len = s.len();
        if len > MAX_SEP_LEN {
            return Err(Error::capacity(len, MAX_SEP_LEN));
        }
        Ok(SeparatorStr(s))
    }

    /// Allows recovery of the initial / wrapped `&str`.
    pub fn into_str(self) -> &'a str {
        self.0
    }
}

impl<'a> fmt::Debug for SeparatorStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<'a> fmt::Display for SeparatorStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
