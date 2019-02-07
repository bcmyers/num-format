#![cfg(feature = "std")]

use arrayvec::ArrayString;
use libc::c_char;

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN};
use crate::utils::{InfinityStr, MinusSignStr, NanStr};
use crate::{Error, Format, Grouping, Locale};

/// Type for obtaining your system locale from the `LC_ALL` environment variable. Implements [`Format`].
///
/// # Example
/// ```rust
/// use std::env;
///
/// use num_format::{Environment, Error, ToFormattedString};
///
/// fn main() -> Result<(), Error> {
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
        use self::platform;
        platform::new_environment()
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
        self.inf = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), MAX_INF_LEN))?;
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
        self.nan = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), MAX_NAN_LEN))?;
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

#[cfg(unix)]
mod platform {
    use crate::utils;

    use super::*;

    pub(crate) fn new_environment() -> Result<Environment, Error> {
        let empty_slice = &['\0' as c_char];
        let _ = unsafe { libc::setlocale(libc::LC_MONETARY, empty_slice.as_ptr()) };

        let ptr = unsafe { libc::localeconv() };
        if ptr.is_null() {
            return Err(Error::c("C function 'localeconv' returned a null pointer."));
        }

        let lconv: &libc::lconv = unsafe { ptr.as_ref() }.unwrap();

        let dec_ptr = utils::Pointer::new(lconv.mon_decimal_point)?;
        let grp_ptr = utils::Pointer::new(lconv.mon_grouping)?;
        let min_ptr = utils::Pointer::new(lconv.negative_sign)?;
        let sep_ptr = utils::Pointer::new(lconv.mon_thousands_sep)?;

        let maybe_dec = dec_ptr.as_char()?;
        let grp = grp_ptr.as_grouping()?;
        let min = min_ptr.as_str()?;
        let maybe_sep = sep_ptr.as_char()?;

        let environment = Environment {
            dec: maybe_dec.unwrap_or_else(|| '.'),
            grp,
            inf: ArrayString::from(Locale::en.infinity()).unwrap(),
            min: ArrayString::from(min).map_err(|_| Error::capacity(min.len(), MAX_MIN_LEN))?,
            nan: ArrayString::from(Locale::en.nan()).unwrap(),
            sep: maybe_sep,
        };

        Ok(environment)
    }
}

#[cfg(windows)]
mod platform {
    use super::*;

    use std::ffi::OsStr;
    use std::marker::PhantomData;
    use std::os::windows::ffi::OsStrExt;

    use widestring::U16CString;
    use winapi::ctypes::{c_int, c_ulong, wchar_t};
    use winapi::um::winnls::{GetNumberFormatEx, GetUserDefaultLocaleName, NUMBERFMTW};

    use crate::windows::{self, LOCALE_NAME_SYSTEM_DEFAULT};

    pub(crate) fn new_environment() -> Result<Environment, Error> {
        // TODO

        let lp_locale_name =
            U16CString::from_str("en-US").map_err(|e| Error::new(&e.to_string()))?;

        let lc_type = windows::LOCALE_SGROUPING;
        let s = get_locale_info_ex(&lp_locale_name, lc_type)?;
        println!("{}", &s);

        let environment = Environment {
            dec: Locale::en.decimal(),
            grp: Locale::en.grouping(),
            inf: ArrayString::from(Locale::en.infinity()).unwrap(),
            min: ArrayString::from(Locale::en.minus_sign()).unwrap(),
            nan: ArrayString::from(Locale::en.nan()).unwrap(),
            sep: Locale::en.separator(),
        };

        Ok(environment)
    }

    fn foo(lc_type: c_ulong) {}

    fn get_locale_info_ex(lp_locale_name: &U16CString, lc_type: c_ulong) -> Result<String, Error> {
        use core::mem;
        use core::ptr;

        use winapi::um::winnls::GetLocaleInfoEx;

        let size = unsafe { GetLocaleInfoEx(lp_locale_name.as_ptr(), lc_type, ptr::null_mut(), 0) };

        let mut buf: Vec<wchar_t> = vec![0; 1024];

        let err =
            unsafe { GetLocaleInfoEx(lp_locale_name.as_ptr(), lc_type, buf.as_mut_ptr(), size) };
        if err == 0 {
            return Err(Error::new("TODO: something went wrong"));
        }

        let windows_string = U16CString::from_vec_with_nul(buf).map_err(|_| Error::new("todo"))?;
        let s = windows_string.to_string().map_err(|_| Error::new("todo"))?;

        Ok(s)
    }

    pub struct WindowsString(Vec<wchar_t>);

    impl WindowsString {
        pub(crate) fn new<S>(s: S) -> Result<WindowsString, Error>
        where
            S: AsRef<str>,
        {
            let s = s.as_ref();
            let os_str = OsStr::new(s);
            let mut vec = os_str.encode_wide().collect::<Vec<wchar_t>>();
            for b in &vec {
                if *b == 0 {
                    return Err(Error::new("TODO: input must not contain any null bytes."));
                }
            }
            vec.push(0);
            Ok(WindowsString(vec))
        }

        pub(crate) fn as_ptr(&self) -> *const wchar_t {
            (&self.0).as_ptr()
        }

        pub(crate) fn as_mut_ptr(&mut self) -> *mut wchar_t {
            (&mut self.0).as_mut_ptr()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_environment_windows() {
            let _ = new_environment().unwrap();
        }
    }
}
