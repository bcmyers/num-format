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
    #![allow(dead_code)] // TODO
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(trivial_casts)]
    include!(concat!(env!("OUT_DIR"), "/xlocale.rs"));
}

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;

// use num_format_common::constants::MAX_MIN_LEN;
use num_format_common::{Grouping, Locale};

use crate::error::Error;
use crate::SystemLocale;

pub(crate) fn new<S>(name: Option<S>) -> Result<SystemLocale, Error>
where
    S: Into<String>,
{
    let name = name.map(|s| s.into());

    println!();
    println!("INPUT: {:?}", &name);

    // create a new locale object
    let name_cstring = match name {
        Some(ref name) => CString::new(name.as_bytes())?,
        None => CString::new("").unwrap(),
    };

    let mask =
        (bindings::LC_CTYPE_MASK | bindings::LC_NUMERIC_MASK | bindings::LC_MONETARY_MASK) as c_int; // TODO
    let new_locale = unsafe { bindings::newlocale(mask, name_cstring.as_ptr(), ptr::null_mut()) };
    if new_locale.is_null() {
        return Err(Error::unix(
            "newlocale unexpectedly returned a null pointer.",
        ));
    }

    // use the new locale object, while saving the initial one
    let initial_locale = unsafe { bindings::uselocale(new_locale) };
    if initial_locale.is_null() {
        panic!("TODO: null ptr")
    }

    // get the encoding
    let encoding_ptr = unsafe { bindings::nl_langinfo_l(bindings::CODESET as c_int, new_locale) };
    match StaticCString::new(encoding_ptr) {
        Ok(val) => {
            println!("ENCODING: {:?} | {:?}", val.to_bytes(), val.to_str());
        }
        Err(e) => {
            // free the new locale object
            let _ = unsafe { bindings::freelocale(new_locale) };
            return Err(e);
        }
    }

    // get the lconv
    let lconv_ptr = unsafe { bindings::localeconv_l(new_locale) };
    if lconv_ptr.is_null() {
        // free the new locale object
        let _ = unsafe { bindings::freelocale(new_locale) };
        return Err(Error::unix("'localeconv_l' returned a null pointer."));
    }
    let lconv: &bindings::lconv = unsafe { lconv_ptr.as_ref() }.unwrap();
    let lconv = match Lconv::new(lconv) {
        Ok(lconv) => lconv,
        Err(e) => {
            // free the new locale object
            let _ = unsafe { bindings::freelocale(new_locale) };
            return Err(e);
        }
    };

    // get the name
    let name = match get_name(mask, name, new_locale) {
        Ok(name) => name,
        Err(e) => {
            // free the new locale object
            let _ = unsafe { bindings::freelocale(new_locale) };
            return Err(e);
        }
    };

    // reset to the initial locale object
    let _ = unsafe { bindings::uselocale(initial_locale) };

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
            let static_c_string = StaticCString::new(name_ptr)?;
            println!(
                "NAME: {:?} | {:?}",
                static_c_string.to_bytes(),
                static_c_string.to_str()
            );
            match static_c_string.to_str() {
                Some(s) => s.to_string(), // TODO
                None => "*****INVALID UTF8****".to_string(),
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

pub struct Lconv {
    dec: char,
    grp: Grouping,
    min: String,
    sep: Option<char>,
}

impl Lconv {
    pub fn new(lconv: &bindings::lconv) -> Result<Lconv, Error> {
        let _dec = {
            let numeric = StaticCString::new(lconv.decimal_point)?;
            let monetary = StaticCString::new(lconv.mon_decimal_point)?;
            print!("DECIMAL_POINT: ");
            if numeric.to_bytes() == monetary.to_bytes() {
                println!("{:?} | {:?}", numeric.to_bytes(), numeric.to_str());
            } else {
                println!();
                println!(
                    "    numeric: {:?} | {:?}",
                    numeric.to_bytes(),
                    numeric.to_str()
                );
                println!(
                    "    monetary: {:?} | {:?}",
                    monetary.to_bytes(),
                    monetary.to_str()
                );
            }
        };

        let _grp = {
            let numeric = StaticCString::new(lconv.grouping)?;
            let monetary = StaticCString::new(lconv.mon_grouping)?;
            print!("GROUPING: ");
            if numeric.to_bytes() == monetary.to_bytes() {
                println!("{:?} | {:?}", numeric.to_bytes(), numeric.to_str());
            } else {
                println!();
                println!(
                    "    numeric: {:?} | {:?}",
                    numeric.to_bytes(),
                    numeric.to_str()
                );
                println!(
                    "    monetary: {:?} | {:?}",
                    monetary.to_bytes(),
                    monetary.to_str()
                );
            }
        };

        let _min = {
            let numeric = StaticCString::new(lconv.negative_sign)?;
            println!(
                "NEGATIVE_SIGN: {:?} | {:?}",
                numeric.to_bytes(),
                numeric.to_str()
            );
        };

        let _sep = {
            let numeric = StaticCString::new(lconv.thousands_sep)?;
            let monetary = StaticCString::new(lconv.mon_thousands_sep)?;
            print!("THOUSANDS_SEP: ");
            if numeric.to_bytes() == monetary.to_bytes() {
                println!("{:?} | {:?}", numeric.to_bytes(), numeric.to_str());
            } else {
                println!();
                println!(
                    "    numeric: {:?} | {:?}",
                    numeric.to_bytes(),
                    numeric.to_str()
                );
                println!(
                    "    monetary: {:?} | {:?}",
                    monetary.to_bytes(),
                    monetary.to_str()
                );
            }
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
    fn new(ptr: *const std::os::raw::c_char) -> Result<StaticCString, Error> {
        if ptr.is_null() {
            return Err(Error::unix("TODO: null pointer."));
        }
        Ok(StaticCString(ptr))
    }

    fn to_bytes(&self) -> &'static [u8] {
        let ptr = self.0;
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let bytes = cstr.to_bytes();
        bytes
    }

    fn to_str(&self) -> Option<&'static str> {
        let bytes = self.to_bytes();
        std::str::from_utf8(bytes).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_locale_bsd() {
        let names = SystemLocale::available_names().unwrap();
        let mut names = names.into_iter().collect::<Vec<String>>();
        names.sort();
        for name in names {
            let _ = SystemLocale::from_name(name.clone()).unwrap();
            std::env::set_var("LC_ALL", &name);
            let _ = SystemLocale::default().unwrap();
        }
    }
}
