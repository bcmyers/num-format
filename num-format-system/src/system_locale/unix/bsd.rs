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
    //! Bindings to xlocale.h.
    //!
    //! * `freelocale`
    //! * `LC_MONETARY_MASK`
    //! * `LC_NUMERIC_MASK`
    //! * TODO
    //! * `newlocale`
    //! * `querylocale`
    //! * `uselocale`
    //!
    //! See build.rs.
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(trivial_casts)]
    include!(concat!(env!("OUT_DIR"), "/xlocale.rs"));
}

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;

use num_format_common::constants::MAX_MIN_LEN;
use num_format_common::{Grouping, Locale};

use crate::error::Error;
use crate::system_locale::unix::{Lconv, Pointer};
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
    // TODO: Worry about sending two values down???
    let mask = (bindings::LC_NUMERIC_MASK | bindings::LC_MONETARY_MASK) as c_int;
    let new_locale = unsafe { bindings::newlocale(mask, name_cstring.as_ptr(), ptr::null_mut()) };
    if new_locale.is_null() {
        return Err(Error::unix(
            "newlocale unexpectedly returned a null pointer.",
        ));
    }

    // use the new locale object, while saving the initial one
    let initial_locale = unsafe { bindings::uselocale(new_locale) };

    // get the encoding
    let encoding_ptr = unsafe { bindings::nl_langinfo_l(bindings::CODESET as c_int, new_locale) };
    if encoding_ptr.is_null() {
        panic!("TODO: null ptr")
    }
    let encoding = match unsafe { CStr::from_ptr(encoding_ptr) }.to_str() {
        Ok(encoding) => encoding,
        Err(_) => {
            // free the new locale object
            let _ = unsafe { bindings::freelocale(new_locale) };
            return Err(Error::unix("TODO55"));
        }
    };
    println!("******* ENCODING: {}", encoding);

    // get the lconv
    let lconv = match localeconv_l(new_locale) {
        Ok(lconv) => lconv,
        Err(e) => {
            // free the new locale object
            let _ = unsafe { bindings::freelocale(new_locale) };
            return Err(e);
        }
    };

    let name = get_name(mask, name, new_locale)?;

    // reset to the initial locale object
    if !initial_locale.is_null() {
        let _ = unsafe { bindings::uselocale(initial_locale) };
    }

    // free the new locale object
    let _ = unsafe { bindings::freelocale(new_locale) };

    Ok(SystemLocale {
        dec: lconv.dec,
        grp: lconv.grp,
        inf: Locale::en.infinity().to_string(),
        min: lconv.min,
        name,
        nan: Locale::en.nan().to_string(),
        sep: lconv.sep,
    })
}

fn get_name(
    mask: c_int,
    name: Option<String>,
    new_locale: bindings::locale_t,
) -> Result<String, Error> {
    let name = match name {
        Some(name) => name,
        None => {
            // don't think we need to free this pointer
            let name_ptr = unsafe { bindings::querylocale(mask, new_locale) };
            if name_ptr.is_null() {
                // free the new locale object
                let _ = unsafe { bindings::freelocale(new_locale) };
                return Err(Error::unix(
                    "querylocale unexpectedly return a null pointer.",
                ));
            }
            let name_cstr = unsafe { CStr::from_ptr(name_ptr) };
            match name_cstr.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => {
                    // free the new locale object
                    let _ = unsafe { bindings::freelocale(new_locale) };
                    return Err(Error::unix(
                        "querylocale unexpected returns string with invalid UTF-8.",
                    ));
                }
            }
        }
    };
    let name = if &name == "POSIX" {
        "C".to_string()
    } else {
        name
    };
    Ok(name)
}

fn localeconv_l(locale: bindings::locale_t) -> Result<Lconv, Error> {
    let ptr = unsafe { bindings::localeconv_l(locale) };
    if ptr.is_null() {
        return Err(Error::unix("'localeconv' returned a null pointer."));
    }

    let lconv: &bindings::lconv = unsafe { ptr.as_ref() }.unwrap();

    let dec_ptr = Pointer::new(lconv.mon_decimal_point)?;
    let grp_ptr = Pointer::new(lconv.mon_grouping)?;
    let min_ptr = Pointer::new(lconv.negative_sign)?;
    let sep_ptr = Pointer::new(lconv.mon_thousands_sep)?;

    let maybe_dec = match dec_ptr.as_char() {
        Ok(maybe_dec) => maybe_dec,
        Err(e) => {
            eprintln!("{}", e);
            Some('.')
        }
    };
    let grp = match grp_ptr.as_grouping() {
        Ok(grp) => grp,
        Err(e) => {
            eprintln!("{}", e);
            Grouping::Standard
        }
    };
    let min = match min_ptr.as_str() {
        Ok(min) => min,
        Err(e) => {
            eprintln!("{}", e);
            "-"
        }
    };
    if min.len() > MAX_MIN_LEN {
        return Err(Error::unix("TODO1"));
    }
    let maybe_sep = match sep_ptr.as_char() {
        Ok(maybe_sep) => maybe_sep,
        Err(e) => {
            eprintln!("{}", e);
            Some(',')
        }
    };

    let locale = Lconv {
        dec: maybe_dec.unwrap_or_else(|| '.'),
        grp,
        min: min.to_string(),
        sep: maybe_sep,
    };

    Ok(locale)
}
