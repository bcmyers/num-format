use arrayvec::ArrayString;

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN, MAX_DEC_LEN, MAX_SEP_LEN};
use crate::{CustomFormat, Error, Format, Grouping, Locale};

/// Type for building [`CustomFormat`]s.
///
/// [`CustomFormat`]: struct.CustomFormat.html
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CustomFormatBuilder {
    dec: Result<ArrayString<[u8; MAX_DEC_LEN]>, Error>,
    grp: Grouping,
    inf: Result<ArrayString<[u8; MAX_INF_LEN]>, Error>,
    min: Result<ArrayString<[u8; MAX_MIN_LEN]>, Error>,
    nan: Result<ArrayString<[u8; MAX_NAN_LEN]>, Error>,
    sep: Result<ArrayString<[u8; MAX_SEP_LEN]>, Error>,
}

impl CustomFormatBuilder {
    pub(crate) fn new() -> Self {
        Self {
            dec: ArrayString::from(Locale::en.decimal()).map_err(|_| unreachable!()),
            grp: Locale::en.grouping(),
            inf: ArrayString::from(Locale::en.infinity()).map_err(|_| unreachable!()),
            min: ArrayString::from(Locale::en.minus_sign()).map_err(|_| unreachable!()),
            nan: ArrayString::from(Locale::en.nan()).map_err(|_| unreachable!()),
            sep: ArrayString::from(Locale::en.separator()).map_err(|_| unreachable!()),
        }
    }

    /// Construct a [`CustomFormat`].
    ///
    /// # Errors
    ///
    /// Return an error if:
    /// - The "infinity sign" is longer than 128 bytes
    /// - The "minus sign" is longer than 7 bytes
    /// - The "nan symbol" is longer than 64 bytes
    ///
    /// [`CustomFormat`]: struct.CustomFormat.html
    pub fn build(self) -> Result<CustomFormat, Error> {
        Ok(CustomFormat {
            dec: self.dec?,
            grp: self.grp,
            inf: self.inf?,
            min: self.min?,
            nan: self.nan?,
            sep: self.sep?,
        })
    }

    /// Sets the character used to represent decimal points.
    pub fn decimal<S>(mut self, value: S) -> Self where S: AsRef<str> {
        let s = value.as_ref();
        self.dec = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), MAX_DEC_LEN));
        self
    }

    /// Sets the decimal, grouping, infinity, minus sign, nan, and separator representations
    /// according to the provided format.
    pub fn format<F>(mut self, value: &F) -> Self
    where
        F: Format,
    {
        let dec_str = value.decimal().into_str();
        let inf_str = value.infinity().into_str();
        let min_str = value.minus_sign().into_str();
        let nan_str = value.nan().into_str();
        let sep_str = value.separator().into_str();

        self.dec = ArrayString::from(dec_str).map_err(|_| unreachable!());
        self.grp = value.grouping();
        self.inf = ArrayString::from(inf_str).map_err(|_| unreachable!());
        self.min = ArrayString::from(min_str).map_err(|_| unreachable!());
        self.nan = ArrayString::from(nan_str).map_err(|_| unreachable!());
        self.sep = ArrayString::from(sep_str).map_err(|_| unreachable!());
        self
    }

    /// Sets the [`Grouping`] used to separate digits.
    ///
    /// [`Grouping`]: enum.Grouping.html
    pub fn grouping(mut self, value: Grouping) -> Self {
        self.grp = value;
        self
    }

    /// Sets the string used for infinity. Note: If the length is greater than 128 bytes
    /// [`build`] will return an error (see [`build`]).
    ///
    /// [`build`]: struct.CustomFormatBuilder.html#method.build
    pub fn infinity<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        let s = value.as_ref();
        self.inf = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), MAX_INF_LEN));
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
        self.min = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), MAX_MIN_LEN));
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
        self.nan = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), MAX_NAN_LEN));
        self
    }

    /// Sets the character, if any, used to represent separtors.
    pub fn separator<S>(mut self, value: S) -> Self where S: AsRef<str> {
        let s = value.as_ref();
        self.sep = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), MAX_SEP_LEN));
        self
    }
}

impl From<CustomFormat> for CustomFormatBuilder {
    fn from(format: CustomFormat) -> CustomFormatBuilder {
        CustomFormatBuilder {
            dec: Ok(format.dec),
            grp: format.grp,
            inf: Ok(format.inf),
            min: Ok(format.min),
            nan: Ok(format.nan),
            sep: Ok(format.sep),
        }
    }
}
