use core::fmt;
use core::ops::Deref;

use arrayvec::ArrayString;

const MAX_DEC_LEN: usize = 8;
const MAX_INF_LEN: usize = 128;
pub(crate) const MAX_MIN_LEN: usize = 8;
const MAX_NAN_LEN: usize = 64;
const MAX_POS_LEN: usize = 8;
pub(crate) const MAX_SEP_LEN: usize = 8;

#[cfg(feature = "with-serde")]
use serde::{de, ser};

use crate::error::Error;

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
        Self::_new(s)
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
        Self::_new(s)
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
        Self::_new(s)
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
        Self::_new(s)
    }
}

/// Simple wrapper type for a `&str` to make sure its length is less than the maximum for
/// a positive sign (8 bytes).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PositiveSignStr<'a>(&'a str);

impl<'a> PositiveSignStr<'a> {
    /// Constructs an [`PositiveSignStr`], ensuring that the length is less than the maximum for
    /// a positive sign (8 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `&str`'s length is more than 8 bytes.
    ///
    /// [`PositiveSignStr`]: struct.PositiveSignStr.html
    pub fn new(s: &'a str) -> Result<PositiveSignStr<'a>, Error> {
        Self::_new(s)
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
        Self::_new(s)
    }
}

macro_rules! create_impls {
    ( $name:ident, $max_len:expr ) => {
        impl<'a> $name<'a> {
            /// Allows recovery of the initial / wrapped `&str`.
            pub fn into_str(self) -> &'a str {
                self.0
            }

            #[inline]
            fn _new(s: &'a str) -> Result<$name<'a>, Error> {
                let len = s.len();
                if len > $max_len {
                    return Err(Error::capacity(len, $max_len));
                }
                Ok($name(s))
            }
        }

        impl<'a> AsRef<str> for $name<'a> {
            fn as_ref(&self) -> &str {
                self.0
            }
        }

        impl<'a> fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }

        impl<'a> fmt::Display for $name<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

create_impls!(DecimalStr, MAX_DEC_LEN);
create_impls!(InfinityStr, MAX_INF_LEN);
create_impls!(MinusSignStr, MAX_MIN_LEN);
create_impls!(NanStr, MAX_NAN_LEN);
create_impls!(PositiveSignStr, MAX_POS_LEN);
create_impls!(SeparatorStr, MAX_SEP_LEN);

macro_rules! create_string {
    ( $name:ident, $visitor:ident, $max_len:expr ) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub(crate) struct $name(ArrayString<[u8; $max_len]>);

        impl $name {
            pub(crate) fn new<S>(s: S) -> Result<Self, Error>
            where
                S: AsRef<str>,
            {
                let s = s.as_ref();
                let a = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), $max_len))?;
                Ok($name(a))
            }

            #[allow(unused)]
            pub(crate) fn max_len() -> usize {
                $max_len
            }
        }

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &str {
                self.0.deref()
            }
        }

        #[cfg(feature = "with-serde")]
        impl ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                serializer.serialize_str(self.0.as_str())
            }
        }

        #[cfg(feature = "with-serde")]
        struct $visitor;

        #[cfg(feature = "with-serde")]
        impl<'de> de::Visitor<'de> for $visitor {
            type Value = $name;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string containing at most {} bytes", $max_len)
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                $name::new(s).map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
            }
        }

        #[cfg(feature = "with-serde")]
        impl<'de> de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor)
            }
        }
    };
}

create_string!(DecString, DecVisitor, MAX_DEC_LEN);
create_string!(InfString, InfVisitor, MAX_INF_LEN);
create_string!(MinString, MinVisitor, MAX_MIN_LEN);
create_string!(NanString, NanVisitor, MAX_NAN_LEN);
create_string!(PosString, PosVisitor, MAX_POS_LEN);
create_string!(SepString, SepVisitor, MAX_SEP_LEN);
