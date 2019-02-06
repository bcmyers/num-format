#![cfg(feature = "std")]

use core::marker::PhantomData;
use core::slice;
use core::str;

use arrayvec::ArrayString;
use libc::{self, c_char};

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN};
use crate::utils::{InfinityStr, MinusSignStr, NanStr};
use crate::{Error, Format, Grouping, Locale};

/// Type for obtaining your system locale from the `LC_ALL` environment variable. Implements [`Format`].
///
/// # Example
/// ```rust
/// use std::env;
///
/// use failure;
/// use num_format::{Environment, ToFormattedString};
///
/// fn main() -> Result<(), failure::Error> {
///     // Use your system's default locale settings
///     let format = Environment::new()?;
///     let s = (-1000000).to_formatted_string(&format);
///     println!("{}", &s);
///
///     // Use your system's locale settings for "en_US.UTF-8"
///     env::set_var("LC_ALL", "en_US.UTF-8");
///     let format = Environment::new()?;
///     let s = (-1000000).to_formatted_string(&format);
///     assert_eq!("-1,000,000", &s);
///
///     Ok(())
/// }
/// ```
///
/// [`Format`]: trait.Format.html
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Environment {
    dec: char,
    grp: Grouping,
    inf: ArrayString<[u8; MAX_INF_LEN]>,
    min: ArrayString<[u8; MAX_MIN_LEN]>,
    nan: ArrayString<[u8; MAX_NAN_LEN]>,
    sep: Option<char>,
}

impl Environment {
    /// Constructs a new [`Environment`].
    ///
    /// # Note
    ///
    /// - This method uses the [`setlocale`] and [`localeconv`] API from C in order to access your
    ///   system's locale settings specified by the `LC_ALL` environment variable.
    /// - Because representations of infinity and NaN are not specified in locale files,
    ///   English representations from the Unicode Common Locale Data Repository (CLDR)
    ///   are used for these values, which are `"∞"` (U+221E) and `"NaN"`. You can change these
    ///   with calls to the [`set_infinity`] and [`set_nan`] methods.
    ///
    /// # Errors
    ///
    /// Return an error if any of the calls into C fail (e.g. if [`localeconv`] return a NULL pointer).
    ///
    /// [`Environment`]: struct.Environment.html
    /// [`localeconv`]: https://www.gnu.org/software/libc/manual/html_node/The-Lame-Way-to-Locale-Data.html#The-Lame-Way-to-Locale-Data
    /// [`setlocale`]: https://www.gnu.org/software/libc/manual/html_node/Setting-the-Locale.html
    /// [`set_infinity`]: struct.Environment.html#method.set_infinity
    /// [`set_nan`]: struct.Environment.html#method.set_nan
    pub fn new() -> Result<Environment, Error> {
        let empty_slice = &['\0' as c_char];
        let _ = unsafe { libc::setlocale(libc::LC_MONETARY, empty_slice.as_ptr()) };

        let ptr = unsafe { libc::localeconv() };
        if ptr.is_null() {
            return Err(Error::c("C function 'localeconv' returned a null pointer"));
        }

        let lconv: &libc::lconv = unsafe { ptr.as_ref() }.unwrap();

        let dec_ptr = SafePointer::new(lconv.mon_decimal_point)?;
        let grp_ptr = SafePointer::new(lconv.mon_grouping)?;
        let min_ptr = SafePointer::new(lconv.negative_sign)?;
        let sep_ptr = SafePointer::new(lconv.mon_thousands_sep)?;

        let maybe_dec = dec_ptr.as_char()?;
        let grp = grp_ptr.as_grouping()?;
        let min = min_ptr.as_str()?;
        let maybe_sep = sep_ptr.as_char()?;

        let environment = Environment {
            dec: maybe_dec.unwrap_or_else(|| '.'),
            grp,
            inf: ArrayString::from(Locale::en.infinity()).unwrap(),
            min: ArrayString::from(min).map_err(|_| Error::capacity(MAX_MIN_LEN))?,
            nan: ArrayString::from(Locale::en.nan()).unwrap(),
            sep: maybe_sep,
        };

        Ok(environment)
    }

    /// Returns this format's representation of decimal points.
    pub fn decimal(&self) -> char {
        self.dec
    }

    /// Returns this format's [`Grouping`], which governs how digits are separated (see [`Grouping`]).
    ///
    /// [`Grouping`]: enum.Grouping.html
    pub fn grouping(&self) -> Grouping {
        self.grp
    }

    /// Returns this format's representation of infinity.
    pub fn infinity(&self) -> &str {
        &self.inf
    }

    /// Returns this format's representation of minus signs.
    pub fn minus_sign(&self) -> &str {
        &self.min
    }

    /// Returns this format's representation of NaN.
    pub fn nan(&self) -> &str {
        &self.nan
    }

    /// Returns this format's representation of separators.
    pub fn separator(&self) -> Option<char> {
        self.sep
    }

    /// Sets the string used for infinity.
    ///
    /// # Errors
    ///
    /// Return an error if the length is greater than 64 bytes.
    pub fn set_infinity<S>(&mut self, value: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        let s = value.as_ref();
        self.inf = ArrayString::from(s).map_err(|_| Error::capacity(MAX_INF_LEN))?;
        Ok(())
    }

    /// Sets the string used for NaN.
    ///
    /// # Errors
    ///
    /// Return an error if the length is greater than 64 bytes.
    pub fn set_nan<S>(&mut self, value: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        let s = value.as_ref();
        self.nan = ArrayString::from(s).map_err(|_| Error::capacity(MAX_NAN_LEN))?;
        Ok(())
    }
}

impl Format for Environment {
    fn decimal(&self) -> char {
        self.decimal()
    }

    fn grouping(&self) -> Grouping {
        self.grouping()
    }

    fn infinity(&self) -> InfinityStr<'_> {
        InfinityStr::new(self.infinity()).unwrap()
    }

    fn minus_sign(&self) -> MinusSignStr<'_> {
        MinusSignStr::new(self.minus_sign()).unwrap()
    }

    fn nan(&self) -> NanStr<'_> {
        NanStr::new(self.nan()).unwrap()
    }

    fn separator(&self) -> Option<char> {
        self.separator()
    }
}

struct SafePointer<'a> {
    ptr: *const c_char,
    phantom: PhantomData<&'a ()>,
}

impl<'a> SafePointer<'a> {
    fn new(ptr: *const c_char) -> Result<SafePointer<'a>, Error> {
        if ptr.is_null() {
            return Err(Error::c("received a null pointer"));
        }
        Ok(SafePointer {
            ptr,
            phantom: PhantomData,
        })
    }

    fn as_char(&self) -> Result<Option<char>, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        let s = str::from_utf8(s)
            .map_err(|_| Error::c("could not parse data returned from C into utf-8"))?;
        if s.chars().count() > 1 {
            return Err(Error::c(
                "received C string of length greater than 1 when C string of length 1 was expected",
            ));
        }
        Ok(s.chars().next())
    }

    fn as_grouping(&self) -> Result<Grouping, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        match s {
            [3, 3] => Ok(Grouping::Standard),
            [3, 2] => Ok(Grouping::Indian),
            [] => Ok(Grouping::Posix),
            _ => Err(Error::c("received unexpected grouping code from C")),
        }
    }

    fn as_str(&self) -> Result<&str, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        let s = str::from_utf8(s)
            .map_err(|_| Error::c("could not parse data returned from C into utf-8"))?;
        if s.len() > MAX_MIN_LEN {
            return Err(Error::capacity(MAX_MIN_LEN));
        }
        Ok(s)
    }
}
