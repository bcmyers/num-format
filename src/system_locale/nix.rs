#![cfg(unix)]

use std::collections::HashSet;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::str;

use crate::constants::MAX_MIN_LEN;
use crate::{Error, Grouping, SystemLocale};

#[allow(dead_code)]
pub(crate) fn default() -> Result<SystemLocale, Error> {
    unimplemented!()
}

#[allow(dead_code)]
pub(crate) fn from_name<S>(name: S) -> Result<SystemLocale, Error>
where
    S: Into<String>,
{
    let _ = name;
    unimplemented!()
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
