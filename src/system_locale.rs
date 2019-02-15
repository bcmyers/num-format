#[cfg(all(feature = "std", any(unix, windows)))]
mod unix;
mod windows;

use std::collections::HashSet;

use crate::error::Error;
use crate::format::Format;
use crate::grouping::Grouping;
use crate::strings::{DecString, InfString, MinString, NanString, PosString, SepString};
use crate::utils::{DecimalStr, InfinityStr, MinusSignStr, NanStr, PositiveSignStr, SeparatorStr};

/// TODO
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SystemLocale {
    pub(crate) dec: DecString,
    pub(crate) grp: Grouping,
    pub(crate) inf: InfString,
    pub(crate) min: MinString,
    pub(crate) name: String,
    pub(crate) nan: NanString,
    pub(crate) pos: PosString,
    pub(crate) sep: SepString,
}

mod todo {
    use std::fmt;

    use super::*;

    impl fmt::Debug for SystemLocale {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fn inner(f: &mut fmt::Formatter, l: &str, s: &str) -> fmt::Result {
                write!(f, "  {}: \"{}\" [", l, s)?;
                for c in s.chars() {
                    for d in c.escape_unicode() {
                        write!(f, "{}", d)?;
                    }
                }
                writeln!(f, "]")
            }

            writeln!(f, "SystemLocale {{")?;
            inner(f, "dec", self.decimal())?;
            writeln!(f, "  grp: {:?}", self.grouping())?;
            inner(f, "min", self.minus_sign())?;
            inner(f, "nam", self.name())?;
            inner(f, "pos", self.positive_sign())?;
            inner(f, "sep", self.separator())?;
            writeln!(f, "}}")?;
            Ok(())
        }
    }
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
                self::unix::new(None)
            }

            /// TODO
            pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
            where
                S: Into<String>,
            {
                self::unix::new(Some(name.into()))
            }

            /// TODO
            pub fn available_names() -> Result<HashSet<String>, Error> {
                Ok(self::unix::available_names())
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
                self::windows::new(None)
            }

            /// TODO
            pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
            where
                S: Into<String>,
            {
                self::windows::new(Some(name))
            }

            /// TODO
            pub fn available_names() -> Result<HashSet<String>, Error> {
                self::windows::available_names()
            }
        }
    }
}

impl SystemLocale {
    /// TODO
    pub fn decimal(&self) -> &str {
        &self.dec
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
    pub fn positive_sign(&self) -> &str {
        &self.pos
    }

    /// TODO
    pub fn separator(&self) -> &str {
        &self.sep
    }

    #[cfg(unix)]
    /// TODO
    pub fn set_infinity<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        self.inf = InfString::new(s)?;
        Ok(())
    }

    #[cfg(unix)]
    /// TODO
    pub fn set_nan<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        self.nan = NanString::new(s)?;
        Ok(())
    }
}

impl std::str::FromStr for SystemLocale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemLocale::from_name(s)
    }
}

impl Format for SystemLocale {
    fn decimal(&self) -> DecimalStr<'_> {
        DecimalStr::new(self.decimal()).unwrap()
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
    fn positive_sign(&self) -> PositiveSignStr<'_> {
        PositiveSignStr::new(self.positive_sign()).unwrap()
    }
    fn separator(&self) -> SeparatorStr<'_> {
        SeparatorStr::new(self.separator()).unwrap()
    }
}
