use crate::utils::{InfinityStr, MinusSignStr, NanStr, DecimalStr, SeparatorStr};
use crate::Grouping;

/// Trait that abstracts over [`CustomFormat`], [`Locale`], and [`SystemLocale`].
///
/// [`CustomFormat`]: struct.CustomFormat.html
/// [`Locale`]: enum.Locale.html
/// [`SystemLocale`]: struct.SystemLocale.html
pub trait Format {
    /// Returns character to use for representing decimal points.
    fn decimal(&self) -> DecimalStr<'_>;
    /// Returns [`Grouping`] to use for separating digits. (see [`Grouping`])
    ///
    /// [`Grouping`]: enum.Grouping.html
    fn grouping(&self) -> Grouping;
    /// Returns string to use for representing infinity symbols.
    fn infinity(&self) -> InfinityStr<'_>;
    /// Returns string to use for representing minus signs.
    fn minus_sign(&self) -> MinusSignStr<'_>;
    /// Returns string to use for representing NaN symbols.
    fn nan(&self) -> NanStr<'_>;
    /// Returns character to use, if any, for representing separators.
    fn separator(&self) -> SeparatorStr<'_>;
}
