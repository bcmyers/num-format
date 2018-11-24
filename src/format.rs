use crate::custom::Custom;
use crate::details::Grouping;
use crate::environment::Environment;
use crate::locale::Locale;

/// Trait that abstracts over `Custom`, `Environment`, and `Locale`.
pub trait Format {
    fn decimal(&self) -> char;
    fn grouping(&self) -> Grouping;
    fn infinity(&self) -> &str;
    fn minus_sign(&self) -> char;
    fn nan(&self) -> &str;
    fn percent_sign(&self) -> &str;
    fn separator(&self) -> char;
}

impl Format for Custom {
    fn decimal(&self) -> char {
        self.decimal()
    }
    fn grouping(&self) -> Grouping {
        self.grouping()
    }
    fn infinity(&self) -> &str {
        self.infinity()
    }
    fn minus_sign(&self) -> char {
        self.minus_sign()
    }
    fn nan(&self) -> &str {
        self.nan()
    }
    fn percent_sign(&self) -> &str {
        self.percent_sign()
    }
    fn separator(&self) -> char {
        self.separator()
    }
}

impl Format for Environment {
    fn decimal(&self) -> char {
        self.decimal()
    }
    fn grouping(&self) -> Grouping {
        self.grouping()
    }
    fn infinity(&self) -> &str {
        "INF"
    }
    fn minus_sign(&self) -> char {
        self.minus_sign()
    }
    fn nan(&self) -> &str {
        "NaN"
    }
    fn percent_sign(&self) -> &str {
        "%"
    }
    fn separator(&self) -> char {
        self.separator()
    }
}

impl Format for Locale {
    fn decimal(&self) -> char {
        self.decimal()
    }
    fn grouping(&self) -> Grouping {
        self.grouping()
    }
    fn infinity(&self) -> &str {
        self.infinity()
    }
    fn minus_sign(&self) -> char {
        self.minus_sign()
    }
    fn nan(&self) -> &str {
        self.nan()
    }
    fn percent_sign(&self) -> &str {
        self.percent_sign()
    }
    fn separator(&self) -> char {
        self.separator()
    }
}
