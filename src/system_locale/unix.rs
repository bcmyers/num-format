#![cfg(unix)]

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::slice;
use std::str;

use crate::constants::MAX_MIN_LEN;
use crate::{Error, Grouping, Locale, SystemLocale};

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

pub(crate) fn default() -> Result<SystemLocale, Error> {
    new(None)
}

pub(crate) fn from_name<S>(name: S) -> Result<SystemLocale, Error>
where
    S: AsRef<str>,
{
    new(Some(name.as_ref()))
}

fn new(name: Option<&str>) -> Result<SystemLocale, Error> {
    let name = setlocale(name)?;
    let lconv = localeconv()?;
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

fn setlocale(name: Option<&str>) -> Result<String, Error> {
    let name_ptr = match name {
        Some(s) => {
            let c_string = CString::new(s).map_err(|_| Error::new("TODO"))?;
            unsafe { libc::setlocale(libc::LC_ALL, c_string.as_ptr()) }
        }
        None => {
            let c_string = &['\0' as libc::c_char];
            unsafe { libc::setlocale(libc::LC_ALL, c_string.as_ptr()) }
        }
    };

    if name_ptr.is_null() {
        return Err(Error::new("TODO"));
    }

    let name_c_str = unsafe { CStr::from_ptr(name_ptr) };
    let name = name_c_str
        .to_str()
        .map_err(|_| Error::new("TODO"))?
        .to_string();

    Ok(name)
}

struct Lconv {
    dec: char,
    grp: Grouping,
    min: String,
    sep: Option<char>,
}

fn localeconv() -> Result<Lconv, Error> {
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
        return Err(Error::new("TODO"));
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

struct Pointer<'a> {
    ptr: *const libc::c_char,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Pointer<'a> {
    fn new(ptr: *const libc::c_char) -> Result<Pointer<'a>, Error> {
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
            .map_err(|_| Error::new("TODO"))?;
        if s.chars().count() > 1 {
            return Err(Error::unix(
                "received C string of length greater than 1 when C string of length 1 was expected",
            ));
        }
        Ok(s.chars().next())
    }

    fn as_grouping(&self) -> Result<Grouping, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        match s {
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
            .map_err(|_| Error::new("TODO"))?;
        if s.len() > MAX_MIN_LEN {
            return Err(Error::capacity(s.len(), MAX_MIN_LEN));
        }
        Ok(s)
    }
}
