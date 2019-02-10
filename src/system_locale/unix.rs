#![cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "openbsd",
    target_os = "netbsd"
))]

mod bindings {
    //! Bindings to xlocale.h.
    //!
    //! * `freelocale`
    //! * `LC_MONETARY_MASK`
    //! * `LC_NUMERIC_MASK`
    //! * `newlocale`
    //! * `querylocale`
    //! * `uselocale`
    //!
    //! See build.rs.
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(trivial_casts)]
    include!(concat!(env!("OUT_DIR"), "/unix.rs"));
}

use std::ffi::{CStr, CString};
use std::ptr;

use crate::system_locale::nix::localeconv;
use crate::{Error, Locale, SystemLocale};

pub(crate) fn default() -> Result<SystemLocale, Error> {
    new::<String>(None)
}

pub(crate) fn from_name<S>(name: S) -> Result<SystemLocale, Error>
where
    S: Into<String>,
{
    new(Some(name))
}

fn new<S>(name: Option<S>) -> Result<SystemLocale, Error>
where
    S: Into<String>,
{
    let name = name.map(|s| s.into());

    // create a new locale object
    let name_cstring = match name {
        Some(ref name) => CString::new(name.as_bytes())?,
        None => CString::new("").unwrap(),
    };
    let mask = (bindings::LC_NUMERIC_MASK | bindings::LC_MONETARY_MASK) as std::os::raw::c_int;
    let new_locale = unsafe { bindings::newlocale(mask, name_cstring.as_ptr(), ptr::null_mut()) };
    if new_locale.is_null() {
        return Err(Error::unix(
            "newlocale unexpectedly returned a null pointer.",
        ));
    }

    // use the new locale object, while saving the initial one
    let initial_locale = unsafe { bindings::uselocale(new_locale) };

    // get the lconv
    let lconv = match localeconv() {
        Ok(lconv) => lconv,
        Err(e) => {
            // free the new locale object
            let _ = unsafe { bindings::freelocale(new_locale) };
            return Err(e);
        }
    };

    // get the name
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
