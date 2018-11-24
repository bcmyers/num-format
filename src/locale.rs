use crate::details::Grouping;

/// Enum representing format settings from the
/// [Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/). Implements `Format`.
#[allow(non_camel_case_types)]
pub enum Locale {
    en,
    fr_FR,
}

impl Locale {
    pub fn decimal(&self) -> char {
        use self::Locale::*;
        match *self {
            en => '.',
            fr_FR => ',',
        }
    }

    pub fn grouping(&self) -> Grouping {
        use self::Locale::*;
        match *self {
            en => Grouping::Standard,
            fr_FR => Grouping::Standard,
        }
    }

    pub fn infinity(&self) -> &'static str {
        use self::Locale::*;
        match *self {
            en => "∞",
            fr_FR => "∞",
        }
    }

    pub fn minus_sign(&self) -> char {
        use self::Locale::*;
        match *self {
            en => '-',
            fr_FR => '-',
        }
    }

    pub fn nan(&self) -> &'static str {
        use self::Locale::*;
        match *self {
            en => "NaN",
            fr_FR => "Nan",
        }
    }

    pub fn percent_sign(&self) -> &'static str {
        use self::Locale::*;
        match *self {
            en => "%",
            fr_FR => "%",
        }
    }

    pub fn separator(&self) -> char {
        use self::Locale::*;
        match *self {
            en => ',',
            fr_FR => '.',
        }
    }
}
