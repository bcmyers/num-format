mod unix;
mod windows;

use std::collections::HashSet;

use cfg_if::cfg_if;
use num_format_common::Grouping;

use crate::error::Error;

/// TODO
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemLocale {
    dec: char,
    grp: Grouping,
    inf: String,
    min: String,
    name: String,
    nan: String,
    sep: Option<char>,
}

cfg_if! {
    if #[cfg(unix)] {
        impl SystemLocale {
            /// TODO
            pub fn new() -> Result<SystemLocale, Error> {
                SystemLocale::default()
            }

            /// TODO
            pub fn default() -> Result<SystemLocale, Error> {
                unix::default()
            }

            /// TODO
            pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
            where
                S: Into<String>,
            {
                unix::from_name(name)
            }

            /// TODO
            pub fn available_names() -> Result<HashSet<String>, Error> {
                Ok(unix::available_names())
            }
        }
    } else {
        impl SystemLocale {
            /// TODO
            pub fn new() -> Result<SystemLocale, Error> {
                SystemLocale::default()
            }

            /// TODO
            pub fn default() -> Result<SystemLocale, Error> {
                windows::default()
            }

            /// TODO
            pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
            where
                S: Into<String>,
            {
                windows::from_name(name)
            }

            // TODO
            pub fn available_names() -> Result<HashSet<String>, Error> {
                windows::available_names()
            }
        }
    }
}

impl SystemLocale {
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
        use num_format_common::MAX_INF_LEN;

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
        use num_format_common::MAX_NAN_LEN;

        let s = s.into();
        if s.len() > MAX_NAN_LEN {
            return Err(Error::new("TODO"));
        }
        self.nan = s;
        Ok(())
    }
}

impl std::str::FromStr for SystemLocale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemLocale::from_name(s)
    }
}
