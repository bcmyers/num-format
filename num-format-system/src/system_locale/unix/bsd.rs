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

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr::{self, NonNull};

use num_format_core::constants::MAX_MIN_LEN;
use num_format_core::{Grouping, Locale};

use crate::encoding::{Encoding, UTF_8};
use crate::error::Error;
use crate::SystemLocale;

pub(crate) fn new(name: Option<String>) -> Result<SystemLocale, Error> {
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
        // get the encoding
        let encoding_ptr =
            unsafe { bindings::nl_langinfo_l(bindings::CODESET as c_int, new_locale) };
        let encoding_static_c_string = StaticCString::new(encoding_ptr, *UTF_8, "nl_langinfo_l")?;
        let encoding = Encoding::from_bytes(&encoding_static_c_string.to_bytes())?;

        // use the new locale object, while saving the initial one
        let initial_locale = unsafe { bindings::uselocale(new_locale) };
        if initial_locale.is_null() {
            return Err(Error::null_ptr("uselocale"));
        }

        // get the lconv
        let lconv_ptr = unsafe { bindings::localeconv_l(new_locale) };
        if lconv_ptr.is_null() {
            return Err(Error::null_ptr("localeconv_l"));
        }
        let lconv: &bindings::lconv = unsafe { lconv_ptr.as_ref() }.unwrap();
        let lconv = Lconv::new(lconv, encoding)?;

        // get the name
        let mut name = match name {
            Some(name) => name,
            None => {
                let name_ptr = unsafe { bindings::querylocale(mask, new_locale) };
                let name_static_c_string = StaticCString::new(name_ptr, encoding, "querylocale")?;
                name_static_c_string.to_string()?
            }
        };
        if &name == "POSIX" {
            name = "C".to_string();
        }

        // reset to the initial locale object
        let _ = unsafe { bindings::uselocale(initial_locale) };

        let system_locale = SystemLocale {
            dec: lconv.dec,
            grp: lconv.grp,
            inf: Locale::en.infinity().to_string(),
            min: lconv.min,
            name,
            nan: Locale::en.nan().to_string(),
            sep: lconv.sep,
        };

        if system_locale.decimal().len() > 1 {
            println!("{:#?}", &system_locale);
        }

        Ok(system_locale)
    };

    let output = inner();

    // free the new locale object
    let _ = unsafe { bindings::freelocale(new_locale) };

    output
}

struct Lconv {
    dec: String,
    grp: Grouping,
    min: String,
    sep: Option<char>,
}

impl Lconv {
    fn new(lconv: &bindings::lconv, encoding: Encoding) -> Result<Lconv, Error> {
        let dec = StaticCString::new(lconv.decimal_point, encoding, "lconv.decimal_point")?
            .to_decimal()?;

        let grp = StaticCString::new(lconv.mon_grouping, encoding, "lconv.mon_grouping")?
            .to_grouping()?;

        let min = StaticCString::new(lconv.negative_sign, encoding, "lconv.negative_sign")?
            .to_minus_sign()?;

        let sep = StaticCString::new(lconv.mon_thousands_sep, encoding, "lconv.mon_thousands_sep")?
            .to_separator()?;

        Ok(Lconv { dec, grp, min, sep })
    }
}

/// Invariants: nul terminated, static lifetime
struct StaticCString {
    encoding: Encoding,
    non_null: NonNull<c_char>,
}

impl StaticCString {
    fn new(
        ptr: *const std::os::raw::c_char,
        encoding: Encoding,
        function_name: &str,
    ) -> Result<StaticCString, Error> {
        let non_null =
            NonNull::new(ptr as *mut c_char).ok_or_else(|| Error::null_ptr(function_name))?;
        Ok(StaticCString { encoding, non_null })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let ptr = self.non_null.as_ptr();
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let bytes = cstr.to_bytes();
        bytes.to_vec()
    }

    fn to_string(&self) -> Result<String, Error> {
        let bytes = self.to_bytes();
        self.encoding.decode(&bytes)
    }

    fn to_decimal(&self) -> Result<String, Error> {
        let s = self.to_string()?;
        if s.len() == 0 {
            return Err(Error::unix(&format!("TODO: Empty decimal: {:?}", &s)));
        }
        Ok(s)
    }

    fn to_grouping(&self) -> Result<Grouping, Error> {
        let bytes = self.to_bytes();
        let bytes: &[u8] = &bytes;
        let grouping = match bytes {
            [3, 2] => Grouping::Indian,
            [] | [127] => Grouping::Posix,
            [3] | [3, 3] => Grouping::Standard,
            _ => return Err(Error::unix(&format!("unsupported grouping: {:?}", bytes))),
        };
        Ok(grouping)
    }

    fn to_minus_sign(&self) -> Result<String, Error> {
        let s = self.to_string()?;
        if s.len() > MAX_MIN_LEN {
            return Err(Error::unix(&format!(
                "TODO: Minus sign longer than max len: {:?} ({})",
                &s,
                s.len(),
            )));
        }
        Ok(s)
    }

    fn to_separator(&self) -> Result<Option<char>, Error> {
        let s = self.to_string()?;
        if s.len() == 0 {
            return Ok(None);
        }
        if s.chars().count() != 1 {
            return Err(Error::unix(&format!(
                "TODO: separator longer than one char: {:?}",
                &s
            )));
        }
        Ok(Some(s.chars().next().unwrap()))
    }
}
