use crate::details::Formatter;
use crate::format::Format;

/// <b>One of the two main traits.</b> Use it to format <b>floats</b> according to
/// a particular `Format`.
pub trait DisplayFloat {
    fn formatter(&self) -> Formatter<'static>;

    /// Returns a `String` representation of the float formatted according to
    /// the default locale (`Locale::en_US`).
    fn to_formatted(&self, decimal_places: usize) -> String {
        self.formatter().decimal_places(decimal_places).to_string()
    }

    /// Returns a `String` representation of the float formatted according to
    /// the provided `Format`, which can either be a `Custom`, an `Environment`, or a `Locale`.
    fn to_formatted_with<F>(&self, format: &F, decimal_places: usize) -> String
    where
        F: Format,
    {
        self.formatter()
            .decimal(format.decimal())
            .decimal_places(decimal_places)
            .grouping(format.grouping())
            .minus_sign(format.minus_sign())
            .percent_sign(format.percent_sign())
            .separator(format.separator())
            .to_string()
    }

    /// Returns a `String` representation of the float formatted <b>as a percent</b> and according
    /// to the default locale (`Locale::en_US`).
    fn to_percent(&self, decimal_places: usize) -> String {
        self.formatter().decimal_places(decimal_places).to_percent()
    }

    /// Returns a `String` representation of the float formatted <b>as a percent</b> and according
    /// to the provided `Format`, which can either be a `Custom`, an `Environment`, or a `Locale`.
    fn to_percent_with<F>(&self, format: &F, decimal_places: usize) -> String
    where
        F: Format,
    {
        self.formatter()
            .decimal(format.decimal())
            .decimal_places(decimal_places)
            .grouping(format.grouping())
            .minus_sign(format.minus_sign())
            .percent_sign(format.percent_sign())
            .separator(format.separator())
            .to_percent()
    }
}
