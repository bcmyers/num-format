use crate::to_formatted_string::ToFormattedString;
use crate::error::Error;
use crate::format::Format;

// TODO: Eliminate FromFormattedStr and put it on ToFormattedString

/// TODO
pub trait ParseFormatted {
    /// A relatively liberal parsing function for turning a formatted `&str` into a number
    /// (any type that implements `ToFormattedString`). The implementation is "liberal" in the
    /// sense that it errs on the side of not failing.
    ///
    /// The underlying algorithm is as follows...
    /// * Look at the first characters of the input to see if they match the provided `Format`s
    ///   minus sign, in which case write `'-'` to a temporary buffer.
    /// * Walk each remaining character, ignoring any characters that are not ascii numeric and
    ///   storing in the temporary buffer any characters that are.
    /// * Call the regular `from_str` method from `FromStr` on the temporary buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_format::{Locale, ParseFormatted};
    ///
    /// fn main() {
    ///     let n = "-1,000,000".parse_formatted::<_, i32>(&Locale::en).unwrap();
    ///     assert_eq!(n, -1_000_000i32);
    ///
    ///     let n = "-1foo0bar0baz0,000".parse_formatted::<_, i32>(&Locale::en).unwrap();
    ///     assert_eq!(n, -1_000_000i32);
    /// }
    /// ```
    fn parse_formatted<F, N>(&self, format: &F) -> Result<N, Error> where F: Format, N: ToFormattedString;
}

impl<S> ParseFormatted for S where S: AsRef<str> {
    fn parse_formatted<F, N>(&self, format: &F) -> Result<N, Error> where F: Format, N: ToFormattedString {
        ToFormattedString::from_formatted_str(self.as_ref(), format)
    }
}
