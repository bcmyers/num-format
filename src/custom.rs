use std::borrow::Cow;

use crate::details::CustomBuilder;
use crate::details::Grouping;

/// Struct representing custom format settings. Use this if you want to specify your own format
/// settings. Implements `Format`.
pub struct Custom {
    pub(crate) decimal: char,
    pub(crate) grouping: Grouping,
    pub(crate) infinity: Cow<'static, str>,
    pub(crate) minus_sign: char,
    pub(crate) nan: Cow<'static, str>,
    pub(crate) percent_sign: Cow<'static, str>,
    pub(crate) separator: char,
}

impl Custom {
    pub fn builder() -> CustomBuilder {
        CustomBuilder::new()
    }
    pub fn decimal(&self) -> char {
        self.decimal
    }
    pub fn grouping(&self) -> Grouping {
        self.grouping
    }
    pub fn infinity(&self) -> &str {
        &self.infinity
    }
    pub fn minus_sign(&self) -> char {
        self.minus_sign
    }
    pub fn nan(&self) -> &str {
        &self.nan
    }
    pub fn percent_sign(&self) -> &str {
        &self.percent_sign
    }
    pub fn separator(&self) -> char {
        self.separator
    }
}
