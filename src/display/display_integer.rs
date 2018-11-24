use crate::details::Formatter;
use crate::format::Format;

/// <b>One of the two main traits.</b> Use it to format <b>integers</b> according to
/// a particular `Format`.
pub trait DisplayInteger {
    fn formatter(&self) -> Formatter<'static>;

    /// Returns a `String` formatted according to the default locale (`Locale::en_US`).
    fn to_formatted(&self) -> String {
        self.formatter().to_string()
    }

    /// Returns a `String` formatted according to the provided `Format`, which can either be a
    /// `Locale` or a `CustomFormat`.
    fn to_formatted_with<F>(&self, format: &F) -> String
    where
        F: Format,
    {
        self.formatter()
            .decimal(format.decimal())
            .grouping(format.grouping())
            .minus_sign(format.minus_sign())
            .separator(format.separator())
            .to_string()
    }
}
