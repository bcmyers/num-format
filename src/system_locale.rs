#![cfg(feature = "std")]

mod unix;
mod windows;

use crate::utils::{InfinityStr, MinusSignStr, NanStr};
use crate::{Format, Grouping};

/// TODO
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SystemLocale {
    dec: char,
    grp: Grouping,
    inf: String,
    min: String,
    nan: String,
    sep: Option<char>,
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
    pub fn nan(&self) -> &str {
        &self.nan
    }

    /// TODO
    pub fn separator(&self) -> Option<char> {
        self.sep
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
