#![cfg(all(
    unix,
    any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "netbsd"
    )
))]

mod bindings {
    //! Bindings to xlocale.h. See build.rs.
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(trivial_casts)]
    include!(concat!(env!("OUT_DIR"), "/bsd.rs"));
}

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;

// use num_format_core::constants::MAX_MIN_LEN;
use num_format_core::{Grouping, Locale};

use crate::encoding::Encoding;
use crate::error::Error;
use crate::SystemLocale;

pub(crate) fn new<S>(name: Option<S>) -> Result<SystemLocale, Error>
where
    S: Into<String>,
{
    let name = name.map(|s| s.into());

    // create a new locale object
    let name_cstring = match name {
        Some(ref name) => CString::new(name.as_bytes())?,
        None => CString::new("").unwrap(),
    };
    let mask =
        (bindings::LC_CTYPE_MASK | bindings::LC_NUMERIC_MASK | bindings::LC_MONETARY_MASK) as c_int;
    let new_locale = unsafe { bindings::newlocale(mask, name_cstring.as_ptr(), ptr::null_mut()) };
    if new_locale.is_null() {
        return Err(Error::null_ptr("newlocale"));
    }

    let inner = || {
        // use the new locale object, while saving the initial one
        let initial_locale = unsafe { bindings::uselocale(new_locale) };
        if initial_locale.is_null() {
            return Err(Error::null_ptr("uselocale"));
        }

        // get the encoding
        let encoding_ptr =
            unsafe { bindings::nl_langinfo_l(bindings::CODESET as c_int, new_locale) };
        let encoding_static_c_string = StaticCString::new(encoding_ptr, "nl_langinfo_l")?;
        let encoding = Encoding::from_bytes(encoding_static_c_string.to_bytes())?;

        // get the lconv
        let lconv_ptr = unsafe { bindings::localeconv_l(new_locale) };
        if lconv_ptr.is_null() {
            return Err(Error::null_ptr("localeconv_l"));
        }
        let lconv: &bindings::lconv = unsafe { lconv_ptr.as_ref() }.unwrap();
        let lconv = Lconv::new(lconv, encoding)?;

        // get the name
        let mut name = match name {
            Some(name) => Cow::Owned(name),
            None => {
                let name_ptr = unsafe { bindings::querylocale(mask, new_locale) };
                let name_static_c_string = StaticCString::new(name_ptr, "querylocale")?;
                let name = name_static_c_string.to_str()?;
                Cow::Borrowed(name)
            }
        };
        if &name == "POSIX" {
            name = Cow::Borrowed("C");
        }

        // reset to the initial locale object
        let _ = unsafe { bindings::uselocale(initial_locale) };

        Ok(SystemLocale {
            dec: lconv.dec,
            grp: lconv.grp,
            inf: Locale::en.infinity().to_string(),
            min: lconv.min,
            name,
            nan: Locale::en.nan().to_string(),
            sep: lconv.sep,
        })
    };

    let output = inner();

    // free the new locale object
    let _ = unsafe { bindings::freelocale(new_locale) };

    output
}

struct Lconv {
    dec: char,
    grp: Grouping,
    min: String,
    sep: Option<char>,
}

impl Lconv {
    fn new(lconv: &bindings::lconv, _encoding: Encoding) -> Result<Lconv, Error> {
        let _dec = {
            let _numeric = StaticCString::new(lconv.decimal_point, "lconv.decimal_point")?;
            let _monetary = StaticCString::new(lconv.mon_decimal_point, "lconv.mon_decimal_point")?;
        };

        let _grp = {
            let _numeric = StaticCString::new(lconv.grouping, "lconv.grouping")?;
            let _monetary = StaticCString::new(lconv.mon_grouping, "lconv.mon_grouping")?;
        };

        let _min = {
            let _numeric = StaticCString::new(lconv.negative_sign, "lconv.negative_sign")?;
        };

        let _sep = {
            let _numeric = StaticCString::new(lconv.thousands_sep, "lconv.thousands_sep")?;
            let _monetary = StaticCString::new(lconv.mon_thousands_sep, "lconv.mon_thousands_sep")?;
        };

        Ok(Lconv {
            dec: '.',
            grp: Grouping::Standard,
            min: "-".to_string(),
            sep: Some(','),
        })
    }
}

/// Invariants: nul terminated, static lifetime
struct StaticCString(*const std::os::raw::c_char);

impl StaticCString {
    fn new(ptr: *const std::os::raw::c_char, function_name: &str) -> Result<StaticCString, Error> {
        if ptr.is_null() {
            return Err(Error::null_ptr(function_name));
        }
        Ok(StaticCString(ptr))
    }

    fn to_bytes(&self) -> &'static [u8] {
        let ptr = self.0;
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let bytes = cstr.to_bytes();
        bytes
    }

    fn to_str(&self) -> Result<&'static str, Error> {
        let bytes = self.to_bytes();
        std::str::from_utf8(bytes).map_err(|_| Error::decoding(bytes, "UTF-8"))
    }
}
