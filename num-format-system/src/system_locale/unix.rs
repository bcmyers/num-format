#![cfg(unix)]

cfg_if! {
    if #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "netbsd"
    ))] {
        mod bsd;
        use self::bsd::{get_encoding, get_lconv, get_name};
    } else {
        mod linux;
        use self::linux::{get_encoding, get_lconv, get_name};
    }
}

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::process::Command;
use std::ptr::{self, NonNull};

use libc::{c_char, c_int, c_void};
use num_format_core::constants::MAX_MIN_LEN;
use num_format_core::{Grouping, Locale};

use crate::encoding::Encoding;
use crate::error::Error;
use crate::system_locale::SystemLocale;

extern "C" {
    pub fn freelocale(locale: *const c_void);
    pub fn newlocale(mask: c_int, name: *const c_char, base: *const c_void) -> *const c_void;
    pub fn uselocale(locale: *const c_void) -> *const c_void;
}

pub(crate) fn available_names() -> HashSet<String> {
    let inner = || {
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
    };
    match inner() {
        Some(set) => set,
        None => HashSet::default(),
    }
}

pub(crate) fn new(maybe_name: Option<String>) -> Result<SystemLocale, Error> {
    // create a new locale object
    let new = new_locale(&maybe_name)?;

    let inner = || {
        // use the new locale object, while saving the initial one
        let initial = use_locale(new)?;

        // get the encoding
        let encoding = get_encoding(new)?;

        // get the lconv
        let lconv = get_lconv(new, encoding)?;

        // get the name
        let mut name = match maybe_name {
            Some(name) => name,
            None => get_name(new, encoding)?,
        };
        if &name == "POSIX" {
            name = "C".to_string();
        }

        // reset to the initial locale object
        let _ = use_locale(initial);

        let system_locale = SystemLocale {
            dec: lconv.dec,
            grp: lconv.grp,
            inf: Locale::en.infinity().to_string(),
            min: lconv.min,
            name,
            nan: Locale::en.nan().to_string(),
            pos: lconv.pos,
            sep: lconv.sep,
        };

        Ok(system_locale)
    };

    let output = inner();

    // free the new locale object
    free_locale(new);

    output
}

fn free_locale(locale: *const c_void) {
    unsafe { freelocale(locale) };
}

fn new_locale(name: &Option<String>) -> Result<*const c_void, Error> {
    let name_cstring = match name {
        Some(ref name) => CString::new(name.as_bytes())?,
        None => CString::new("").unwrap(),
    };
    let mask = libc::LC_CTYPE_MASK | libc::LC_MONETARY_MASK | libc::LC_NUMERIC_MASK;
    let new_locale = unsafe { newlocale(mask, name_cstring.as_ptr(), ptr::null()) };
    if new_locale.is_null() {
        return Err(Error::null_ptr("newlocale"));
    }
    Ok(new_locale)
}

fn use_locale(locale: *const c_void) -> Result<*const c_void, Error> {
    let old_locale = unsafe { uselocale(locale) };
    if old_locale.is_null() {
        return Err(Error::null_ptr("uselocale"));
    }
    Ok(old_locale)
}

pub(crate) struct Lconv {
    pub(crate) dec: String,
    pub(crate) grp: Grouping,
    pub(crate) min: String,
    pub(crate) pos: String,
    pub(crate) sep: Option<String>,
}

impl Lconv {
    pub(crate) fn new(lconv: &libc::lconv, encoding: Encoding) -> Result<Lconv, Error> {
        let dec = {
            let s = StaticCString::new(lconv.decimal_point, encoding, "lconv.decimal_point")?
                .to_string()?;
            if s.len() == 0 {
                return Err(Error::unix(&format!("TODO: Empty decimal: {:?}", &s)));
            }
            s
        };

        let grp = StaticCString::new(lconv.grouping, encoding, "lconv.grouping")?.to_grouping()?;

        let min = {
            let s = StaticCString::new(lconv.negative_sign, encoding, "lconv.negative_sign")?
                .to_string()?;
            if s.len() > MAX_MIN_LEN {
                return Err(Error::unix(&format!(
                    "TODO: Minus sign longer than max len: {:?} ({})",
                    &s,
                    s.len(),
                )));
            }
            s
        };

        let pos = StaticCString::new(lconv.positive_sign, encoding, "lconv.positive_sign")?
            .to_string()?;

        let sep = {
            let s = StaticCString::new(lconv.thousands_sep, encoding, "lconv.thousands_sep")?
                .to_string()?;
            if s.len() == 0 {
                None
            } else {
                Some(s)
            }
        };

        Ok(Lconv {
            dec,
            grp,
            min,
            pos,
            sep,
        })
    }
}

/// Invariants: nul terminated, static lifetime
pub(crate) struct StaticCString {
    encoding: Encoding,
    non_null: NonNull<c_char>,
}

impl StaticCString {
    pub(crate) fn new(
        ptr: *const std::os::raw::c_char,
        encoding: Encoding,
        function_name: &str,
    ) -> Result<StaticCString, Error> {
        let non_null =
            NonNull::new(ptr as *mut c_char).ok_or_else(|| Error::null_ptr(function_name))?;
        Ok(StaticCString { encoding, non_null })
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let ptr = self.non_null.as_ptr();
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let bytes = cstr.to_bytes();
        bytes.to_vec()
    }

    pub(crate) fn to_grouping(&self) -> Result<Grouping, Error> {
        let bytes = self.to_bytes();
        let bytes: &[u8] = &bytes;
        let grouping = match bytes {
            [3, 2] | [2, 3] => Grouping::Indian, // TODO
            [] | [127] => Grouping::Posix,
            [3] | [3, 3] => Grouping::Standard,
            _ => return Err(Error::unix(&format!("unsupported grouping: {:?}", bytes))),
        };
        Ok(grouping)
    }

    pub(crate) fn to_string(&self) -> Result<String, Error> {
        let bytes = self.to_bytes();
        self.encoding.decode(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unix_available_names() {
        let set = available_names();
        assert!(!set.is_empty());
    }
}
