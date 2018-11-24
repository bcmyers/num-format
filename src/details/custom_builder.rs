use std::borrow::Cow;

use crate::custom::Custom;
use crate::details::Grouping;

pub struct CustomBuilder {
    decimal: char,
    grouping: Grouping,
    infinity: Cow<'static, str>,
    minus_sign: char,
    nan: Cow<'static, str>,
    percent_sign: Cow<'static, str>,
    separator: char,
}

impl CustomBuilder {
    pub(crate) fn new() -> Self {
        CustomBuilder {
            decimal: '.',
            grouping: Grouping::Standard,
            infinity: "∞".into(),
            minus_sign: '-',
            nan: "NaN".into(),
            percent_sign: "%".into(),
            separator: ',',
        }
    }

    pub fn build(self) -> Custom {
        Custom {
            decimal: self.decimal,
            grouping: self.grouping,
            infinity: self.infinity,
            minus_sign: self.minus_sign,
            nan: self.nan,
            percent_sign: self.percent_sign,
            separator: self.separator,
        }
    }

    pub fn decimal(mut self, value: char) -> Self {
        self.decimal = value;
        self
    }

    pub fn grouping(mut self, value: Grouping) -> Self {
        self.grouping = value;
        self
    }

    pub fn infinity<S>(mut self, value: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.infinity = value.into();
        self
    }

    pub fn minus_sign(mut self, value: char) -> Self {
        self.minus_sign = value;
        self
    }

    pub fn nan<S>(mut self, value: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.nan = value.into();
        self
    }

    pub fn percent_sign<S>(mut self, value: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.percent_sign = value.into();
        self
    }

    pub fn separator(mut self, value: char) -> Self {
        self.separator = value;
        self
    }
}
