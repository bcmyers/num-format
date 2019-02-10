#![cfg(unix)]

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr;
use std::str;

use cfg_if::cfg_if;

use crate::constants::MAX_MIN_LEN;
use crate::{Error, Grouping, Locale, SystemLocale};

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

    let name = get_name(name)?;

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

cfg_if! {
    if #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "netbsd"
    ))] {
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
            include!(concat!(env!("OUT_DIR"), "/xlocale.rs"));
        }

        fn get_name(name: Option<String>) -> Result<String, Error> {
            match name {
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
            }
        }
    } else {
        mod bindings {
            //! Bindings to locale.h.
            //!
            //! * `freelocale`
            //! * `LC_MONETARY_MASK`
            //! * `LC_NUMERIC_MASK`
            //! * `newlocale`
            //! * `uselocale`
            //!
            //! See build.rs.
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]
            #![allow(trivial_casts)]
            include!(concat!(env!("OUT_DIR"), "/locale.rs"));
        }

        fn get_name(name: Option<String>) -> Result<String, Error> {
            let name = match name {
                Some(name) => name,
                None => "TODO".to_string(),
            };
            Ok(name)
        }
    }
}

pub(crate) fn available_names() -> HashSet<String> {
    fn first_attempt() -> Option<HashSet<String>> {
        use std::process::Command;

        let output = Command::new("locale").arg("-a").output().ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = std::str::from_utf8(&output.stdout).ok()?;
        let set = stdout
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<HashSet<String>>();
        Some(set)
    }

    // TODO: Test that these give the same output
    fn second_attempt() -> HashSet<String> {
        use walkdir::WalkDir;

        const LOCALE_DIR: &str = "/usr/share/locale";

        let mut names = WalkDir::new(LOCALE_DIR)
            .max_depth(1)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let metadata = entry.metadata().ok()?;
                if metadata.is_dir() {
                    let path = entry.path().join("LC_NUMERIC");
                    if path.exists() {
                        return Some(entry.file_name().to_str().unwrap().to_string());
                    }
                    let path = entry.path().join("LC_MONETARY");
                    if path.exists() {
                        return Some(entry.file_name().to_str().unwrap().to_string());
                    }
                }
                None
            })
            .collect::<HashSet<String>>();
        let _ = names.insert("C".into());
        let _ = names.insert("POSIX".into());
        names
    }

    match first_attempt() {
        Some(set) => set,
        None => second_attempt(),
    }
}

// TODO: note on how this is a safe wrapper
pub(crate) fn localeconv() -> Result<Lconv, Error> {
    // Note: We do **not** free ptr, as "the localeconv() function returns a pointer to a static
    // object which may be altered by later calls to setlocale(3) or localeconv()."
    // See https://www.freebsd.org/cgi/man.cgi?query=localeconv.
    let ptr = unsafe { libc::localeconv() };
    if ptr.is_null() {
        return Err(Error::unix("'localeconv' returned a null pointer."));
    }

    let lconv: &libc::lconv = unsafe { ptr.as_ref() }.unwrap();

    let dec_ptr = Pointer::new(lconv.mon_decimal_point)?;
    let grp_ptr = Pointer::new(lconv.mon_grouping)?;
    let min_ptr = Pointer::new(lconv.negative_sign)?;
    let sep_ptr = Pointer::new(lconv.mon_thousands_sep)?;

    let maybe_dec = dec_ptr.as_char()?;
    let grp = grp_ptr.as_grouping()?;
    let min = min_ptr.as_str()?;
    if min.len() > MAX_MIN_LEN {
        return Err(Error::unix("TODO"));
    }
    let maybe_sep = sep_ptr.as_char()?;

    let locale = Lconv {
        dec: maybe_dec.unwrap_or_else(|| '.'),
        grp,
        min: min.to_string(),
        sep: maybe_sep,
    };

    Ok(locale)
}

#[derive(Clone, Debug)]
pub(crate) struct Lconv {
    pub(crate) dec: char,
    pub(crate) grp: Grouping,
    pub(crate) min: String,
    pub(crate) sep: Option<char>,
}

#[derive(Debug)]
struct Pointer<'a> {
    ptr: *const std::os::raw::c_char,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Pointer<'a> {
    fn new(ptr: *const std::os::raw::c_char) -> Result<Pointer<'a>, Error> {
        if ptr.is_null() {
            return Err(Error::unix("received a null pointer from C."));
        }
        Ok(Pointer {
            ptr,
            phantom: PhantomData,
        })
    }

    fn as_char(&self) -> Result<Option<char>, Error> {
        let s = unsafe { CStr::from_ptr(self.ptr) }
            .to_str()
            .map_err(|_| Error::unix("TODO"))?;
        if s.chars().count() > 1 {
            return Err(Error::unix(
                "received C string of length greater than 1 when C string of length 1 was expected",
            ));
        }
        Ok(s.chars().next())
    }

    fn as_grouping(&self) -> Result<Grouping, Error> {
        let s = unsafe { CStr::from_ptr(self.ptr) };
        match s.to_bytes() {
            [3] | [3, 3] => Ok(Grouping::Standard),
            [3, 2] => Ok(Grouping::Indian),
            [] => Ok(Grouping::Posix),
            _ => Err(Error::unix(&format!(
                "received unexpected grouping code from C: {:?}",
                s
            ))),
        }
    }

    fn as_str(&self) -> Result<&str, Error> {
        let s = unsafe { CStr::from_ptr(self.ptr) }
            .to_str()
            .map_err(|_| Error::unix("TODO"))?;
        if s.len() > MAX_MIN_LEN {
            return Err(Error::capacity(s.len(), MAX_MIN_LEN));
        }
        Ok(s)
    }
}
