#![cfg(unix)]

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr;
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
    // TODO: need a lock?

    // save current locale settings
    let start = setlocale(Action::Get)?;

    // temporarily set locale settings to what we want
    let name = {
        match name {
            Some(name) => setlocale(Action::SetFromName(name))?,
            None => setlocale(Action::SetFromEnvironment)?,
        }
    };

    // get the locale information
    let lconv = localeconv()?;

    // revert locale settings back to their original settings
    let end = setlocale(Action::SetFromName(&start))?;
    debug_assert_eq!(&start, &end);

    // return the locale information
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

// TODO: note on how this is a safe wrapper
fn localeconv() -> Result<Lconv, Error> {
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

// TODO: note on how this is a safe wrapper
fn setlocale<'a>(action: Action<'a>) -> Result<String, Error> {
    // TODO: Note on why we don't free the pointer.
    let ptr = match action {
        Action::Get => unsafe { libc::setlocale(libc::LC_ALL, ptr::null()) },
        Action::SetFromEnvironment => {
            let cstring = CString::new("").unwrap();
            unsafe { libc::setlocale(libc::LC_ALL, cstring.as_ptr()) }
        }
        Action::SetFromName(name) => {
            let cstring = CString::new(name).map_err(|_| Error::unix("TODO1"))?;
            unsafe { libc::setlocale(libc::LC_ALL, cstring.as_ptr()) }
        }
    };
    if ptr.is_null() {
        match action {
            Action::SetFromName(name) => {
                return Err(Error::unix(format!("locale name {} unavailable.", name)));
            }
            _ => return Err(Error::unix("TODO2")),
        }
    }
    let cstr = unsafe { CStr::from_ptr(ptr) };
    let output = cstr
        .to_str()
        .map_err(|_| Error::unix("value returned from libc::setlocale contains invalid UTF-8."))?
        .to_string();
    Ok(output)
}

#[derive(Clone, Debug)]
enum Action<'a> {
    Get,
    SetFromEnvironment,
    SetFromName(&'a str),
}

#[derive(Clone, Debug)]
struct Lconv {
    dec: char,
    grp: Grouping,
    min: String,
    sep: Option<char>,
}

#[derive(Debug)]
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
            .map_err(|_| Error::unix("TODO"))?;
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
            .map_err(|_| Error::unix("TODO"))?;
        if s.len() > MAX_MIN_LEN {
            return Err(Error::capacity(s.len(), MAX_MIN_LEN));
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::SystemLocale;

    #[test]
    fn test_system_locale_unix_setlocale() {
        let locale = SystemLocale::default().unwrap();
        println!("{:#?}", &locale);
        let locale = SystemLocale::from_name("en_US").unwrap();
        println!("{:#?}", &locale);
        let locale = SystemLocale::from_name("fr_FR").unwrap();
        println!("{:#?}", &locale);
    }
}
