use arrayvec::ArrayString;

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN};
use crate::errors::Error;
use crate::format::{CustomFormat, Format, Grouping, Locale};

/// Type for building [`CustomFormat`]s.
///
/// [`CustomFormat`]: struct.CustomFormat.html
#[derive(Copy, Clone, Debug)]
pub struct CustomFormatBuilder {
    dec: char,
    grp: Grouping,
    inf: Result<ArrayString<[u8; MAX_INF_LEN]>, Error>,
    min: Result<ArrayString<[u8; MAX_MIN_LEN]>, Error>,
    nan: Result<ArrayString<[u8; MAX_NAN_LEN]>, Error>,
    sep: Option<char>,
}

impl CustomFormatBuilder {
    pub(crate) fn new() -> Self {
        Self {
            dec: Locale::en.decimal(),
            grp: Locale::en.grouping(),
            inf: ArrayString::from(Locale::en.infinity()).map_err(|_| Error::capacity(MAX_INF_LEN)),
            min: ArrayString::from(Locale::en.minus_sign())
                .map_err(|_| Error::capacity(MAX_MIN_LEN)),
            nan: ArrayString::from(Locale::en.nan()).map_err(|_| Error::capacity(MAX_NAN_LEN)),
            sep: Locale::en.separator(),
        }
    }

    /// Construct a [`CustomFormat`].
    ///
    /// # Errors
    ///
    /// Return an error if:
    /// - The "infinity sign" is longer than 64 bytes
    /// - The "minus sign" is longer than 7 bytes
    /// - The "nan symbol" is longer than 64 bytes
    ///
    /// [`CustomFormat`]: struct.CustomFormat.html
    pub fn build(self) -> Result<CustomFormat, Error> {
        Ok(CustomFormat {
            dec: self.dec,
            grp: self.grp,
            inf: self.inf?,
            min: self.min?,
            nan: self.nan?,
            sep: self.sep,
        })
    }

    /// Sets the character used to represent decimal points.
    pub fn decimal(mut self, value: char) -> Self {
        self.dec = value;
        self
    }

    /// Sets the decimal, grouping, infinity, minus sign, nan, and separator representations
    /// according to the provided format.
    pub fn format<F>(mut self, value: &F) -> Self
    where
        F: Format,
    {
        self.dec = value.decimal();
        self.grp = value.grouping();
        self.inf = ArrayString::from(value.infinity().into_str())
            .map_err(|_| Error::capacity(MAX_INF_LEN));
        self.min = ArrayString::from(value.minus_sign().into_str())
            .map_err(|_| Error::capacity(MAX_MIN_LEN));
        self.nan =
            ArrayString::from(value.nan().into_str()).map_err(|_| Error::capacity(MAX_NAN_LEN));
        self.sep = value.separator();
        self
    }

    /// Sets the [`Grouping`] used to separate digits.
    ///
    /// [`Grouping`]: enum.Grouping.html
    pub fn grouping(mut self, value: Grouping) -> Self {
        self.grp = value;
        self
    }

    /// Sets the string used for infinity. Note: If the length is greater than 64 bytes
    /// [`build`] will return an error (see [`build`]).
    ///
    /// [`build`]: struct.CustomFormatBuilder.html#method.build
    pub fn infinity<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        let s = value.as_ref();
        self.inf = ArrayString::from(s).map_err(|_| Error::capacity(MAX_INF_LEN));
        self
    }

    /// Sets the string used for minus signs. Note: If the length is greater than 7 bytes
    /// [`build`] will return an error (see [`build`]).
    ///
    /// [`build`]: struct.CustomFormatBuilder.html#method.build
    pub fn minus_sign<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        let s = value.as_ref();
        self.min = ArrayString::from(s).map_err(|_| Error::capacity(MAX_MIN_LEN));
        self
    }

    /// Sets the string used for NaN. Note: If the length is greater than 64 bytes
    /// [`build`] will return an error (see [`build`]).
    ///
    /// [`build`]: struct.CustomFormatBuilder.html#method.build
    pub fn nan<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        let s = value.as_ref();
        self.nan = ArrayString::from(s).map_err(|_| Error::capacity(MAX_NAN_LEN));
        self
    }

    /// Sets the character, if any, used to represent separtors.
    pub fn separator(mut self, value: Option<char>) -> Self {
        self.sep = value;
        self
    }
}

impl From<CustomFormat> for CustomFormatBuilder {
    fn from(format: CustomFormat) -> CustomFormatBuilder {
        CustomFormatBuilder {
            dec: format.dec,
            grp: format.grp,
            inf: Ok(format.inf),
            min: Ok(format.min),
            nan: Ok(format.nan),
            sep: format.sep,
        }
    }
}
