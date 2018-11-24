use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Locale {
    pub(crate) identifier: String,
    pub(crate) language: String,
    pub(crate) script: Option<String>,
    pub(crate) territory: Option<String>,
    pub(crate) variant: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Policy {
    pub(crate) decimal: String,
    pub(crate) decimal_formats: Grouping,
    pub(crate) group: String,
    pub(crate) infinity: String,
    pub(crate) minus_sign: String,
    pub(crate) nan: String,
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Grouping::India => "#,##,##0.###",
            Grouping::Standard => "#,##0.###",
            Grouping::Posix => "0.######",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Grouping {
    Standard,
    India,
    Posix,
}

impl FromStr for Grouping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = match s {
            "#,##0.###" => Grouping::Standard,
            "0.######" => Grouping::Posix,
            "#,##,##0.###" => Grouping::India,
            _ => return Err("Could not parse format".to_string()),
        };
        Ok(format)
    }
}
