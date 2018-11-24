use std::fmt;
use std::str::FromStr;

use proc_macro2::{Ident, Span};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Grouping {
    Standard,
    Indian,
    Posix,
}

impl Grouping {
    pub(crate) fn to_ident(&self) -> Ident {
        match self {
            Grouping::Indian => Ident::new("Indian", Span::call_site()),
            Grouping::Standard => Ident::new("Standard", Span::call_site()),
            Grouping::Posix => Ident::new("Posix", Span::call_site()),
        }
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Grouping::Indian => "#,##,##0.###",
            Grouping::Standard => "#,##0.###",
            Grouping::Posix => "0.######",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Grouping {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = match s {
            "#,##0.###" => Grouping::Standard,
            "0.######" => Grouping::Posix,
            "#,##,##0.###" => Grouping::Indian,
            _ => return Err(failure::format_err!("Could not parse {} into Grouping", s)),
        };
        Ok(format)
    }
}
