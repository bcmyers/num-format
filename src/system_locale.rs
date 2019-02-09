#![cfg(feature = "std")]

mod unix;
mod windows;

use std::collections::HashSet;

use crate::utils::{InfinityStr, MinusSignStr, NanStr};
use crate::{Error, Format, Grouping};

/// TODO
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SystemLocale {
    dec: char,
    grp: Grouping,
    inf: String,
    min: String,
    name: String,
    nan: String,
    sep: Option<char>,
}

impl SystemLocale {
    /// TODO
    pub fn new() -> Result<SystemLocale, Error> {
        SystemLocale::default()
    }

    /// TODO
    pub fn default() -> Result<SystemLocale, Error> {
        #[cfg(unix)]
        {
            return unix::default();
        }

        #[cfg(windows)]
        {
            return windows::default();
        }

        #[cfg(not(unix))]
        #[cfg(not(windows))]
        {
            return Err(Error::new("TODO"));
        }
    }

    /// TODO
    pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
    where
        S: AsRef<str>,
    {
        #[cfg(unix)]
        {
            return unix::from_name(name);
        }

        #[cfg(windows)]
        {
            return windows::from_name(name);
        }

        #[cfg(not(unix))]
        #[cfg(not(windows))]
        {
            return Err(Error::new("TODO"));
        }
    }

    /// TODO
    pub fn available_names() -> Result<HashSet<String>, Error> {
        #[cfg(unix)]
        {
            return Ok(unix::available_names());
        }

        #[cfg(windows)]
        {
            return windows::available_names();
        }

        #[cfg(not(unix))]
        #[cfg(not(windows))]
        {
            return Ok(HashSet::default());
        }
    }

    /// TODO
    pub fn decimal(&self) -> char {
        self.dec
    }

    /// TODO
    pub fn grouping(&self) -> Grouping {
        self.grp
    }

    /// TODO
    pub fn infinity(&self) -> &str {
        &self.inf
    }

    /// TODO
    pub fn minus_sign(&self) -> &str {
        &self.min
    }

    /// TODO
    pub fn name(&self) -> &str {
        &self.name
    }

    /// TODO
    pub fn nan(&self) -> &str {
        &self.nan
    }

    /// TODO
    pub fn separator(&self) -> Option<char> {
        self.sep
    }

    #[cfg(unix)]
    /// TODO
    pub fn set_infinity<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        use crate::constants::MAX_INF_LEN;

        let s = s.into();
        if s.len() > MAX_INF_LEN {
            return Err(Error::new("TODO"));
        }
        self.nan = s;
        Ok(())
    }

    #[cfg(unix)]
    /// TODO
    pub fn set_nan<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        use crate::constants::MAX_NAN_LEN;

        let s = s.into();
        if s.len() > MAX_NAN_LEN {
            return Err(Error::new("TODO"));
        }
        self.nan = s;
        Ok(())
    }
}

impl Format for SystemLocale {
    fn decimal(&self) -> char {
        self.decimal()
    }

    fn grouping(&self) -> Grouping {
        self.grouping()
    }

    fn infinity(&self) -> InfinityStr<'_> {
        InfinityStr::new(self.infinity()).unwrap()
    }

    fn minus_sign(&self) -> MinusSignStr<'_> {
        MinusSignStr::new(self.minus_sign()).unwrap()
    }

    fn nan(&self) -> NanStr<'_> {
        NanStr::new(self.nan()).unwrap()
    }

    fn separator(&self) -> Option<char> {
        self.separator()
    }
}

impl std::str::FromStr for SystemLocale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemLocale::from_name(s)
    }
}
