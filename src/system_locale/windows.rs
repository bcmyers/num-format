#![cfg(windows)]

mod bindings;

use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use lazy_static::lazy_static;
use widestring::{U16CStr, U16CString};
use winapi::shared::minwindef::{BOOL, DWORD, LPARAM};
use winapi::um::winnls;
use winapi::um::winnt::WCHAR;

use self::bindings::{Request, LOCALE_NAME_SYSTEM_DEFAULT, LOCALE_NAME_MAX_LENGTH};
use crate::{Error, Grouping, SystemLocale};
use crate::constants::{MAX_MIN_LEN, MAX_INF_LEN, MAX_NAN_LEN};

impl SystemLocale {
    /// TODO
    pub fn new() -> Result<SystemLocale, Error> {
        SystemLocale::default()
    }

    /// TODO
    pub fn default() -> Result<SystemLocale, Error> {
        SystemLocale::from_name(&*LOCALE_NAME_SYSTEM_DEFAULT)
    }

    /// TODO
    pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref();
        if name.len() > LOCALE_NAME_MAX_LENGTH - 1 {
            return Err(Error::new("TODO"))
        }

        let dec = {
            let dec_string = get_locale_info_ex(name, Request::Decimal)?;
            if dec_string.chars().count() != 1 {
                return Err(Error::new("TODO"))
            }
            dec_string.chars().nth(0).unwrap()
        };

        let grp = {
            let grp_string = get_locale_info_ex(name, Request::Grouping)?;
            match grp_string.as_ref() {
                "3;0" => Grouping::Standard,
                "3" => Grouping::Standard,
                "3;2;0" => Grouping::Indian,
                "3;2" => Grouping::Indian,
                "" => Grouping::Posix,
                _ => return Err(Error::new("TODO"))
            }
        };

        let inf = {
            let inf_string = get_locale_info_ex(name, Request::PositiveInfinity)?;
            if inf_string.len() > MAX_INF_LEN {
                return Err(Error::new("TODO: INF FAILED"))
            }
            inf_string
        };

        let min = {
            let min_string = get_locale_info_ex(name, Request::MinusSign)?;
            if min_string.len() > MAX_MIN_LEN {
                return Err(Error::new("TODO"))
            }
            min_string
        };

        let nan = {
            let nan_string = get_locale_info_ex(name, Request::Nan)?;
            if nan_string.len() > MAX_NAN_LEN {
                return Err(Error::new("TODO: NAN FAILED"))
            }
            nan_string
        };

        let sep = {
            let sep_string = get_locale_info_ex(name, Request::Separator)?;
            match sep_string.chars().count() {
                0 => None,
                1 => Some(sep_string.chars().nth(0).unwrap()),
                _ => return Err(Error::new("TODO"))
            }
        };

        let locale = SystemLocale {
            dec,
            grp,
            inf,
            min,
            nan,
            sep,
        };

        Ok(locale)
    }

    /// TODO
    pub fn available_names() -> Result<HashSet<String>, Error> {
        enum_system_locales_ex()
    }
}

impl std::str::FromStr for SystemLocale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemLocale::from_name(s)
    }
}

fn enum_system_locales_ex() -> Result<HashSet<String>, Error> {
    lazy_static! {
        static ref OUTER_MUTEX: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
        static ref INNER_MUTEX: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::default()));
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn lpLocaleEnumProcEx(lpLocaleName: *mut WCHAR, _: DWORD, _: LPARAM) -> BOOL {
        const CONTINUE: BOOL = 1;
        const STOP: BOOL = 0;

        if lpLocaleName.is_null() {
            return STOP;
        }

        let s = match U16CStr::from_ptr_str(lpLocaleName).to_string() {
            Ok(s) => s,
            Err(_) => return CONTINUE,
        };

        if &s == "" {
            return CONTINUE;
        }

        let mut inner_guard = INNER_MUTEX.lock().unwrap();
        let _ = inner_guard.insert(s);

        CONTINUE
    };

    let set = {
        let outer_guard = OUTER_MUTEX.lock().unwrap();
        {
            let mut inner_guard = INNER_MUTEX.lock().unwrap();
            inner_guard.clear();
        }
        let err = unsafe { winnls::EnumSystemLocalesEx(Some(lpLocaleEnumProcEx), 0, 0, ptr::null_mut()) };
        if err == 0 {
            return Err(Error::new("TODO"));
        }
        let set = {
            let inner_guard = INNER_MUTEX.lock().unwrap();
            inner_guard.clone()
        };
        drop(outer_guard);
        set
    };

    Ok(set)
}

fn get_locale_info_ex(locale_name: &str, request: Request) -> Result<String, Error> {
    const BUF_LEN: usize = 256;

    let locale_name = U16CString::from_str(locale_name).map_err(|_| Error::new("TODO1"))?;

    #[allow(non_snake_case)]
    let lpLocaleName = locale_name.as_ptr();

    #[allow(non_snake_case)]
    let LCType = request.into();

    let size = unsafe { winnls::GetLocaleInfoEx(lpLocaleName, LCType, ptr::null_mut(), 0) };
    if size <= 0 {
        return Err(Error::new("TODO2"));
    }
    // cast is OK because `size` is a c_int (i32) and we've already checked that it's positive
    if size as usize > BUF_LEN {
        return Err(Error::new("TODO3"));
    }

    let mut buf: [WCHAR; BUF_LEN] = unsafe { mem::uninitialized() };
    let err = unsafe { winnls::GetLocaleInfoEx(lpLocaleName, LCType, buf.as_mut_ptr(), size) };
    if err == 0 {
        return Err(Error::new("TODO4"));
    }

    let s = U16CStr::from_slice_with_nul(&buf[..])
        .map_err(|_| Error::new("TODO5"))?
        .to_string()
        .map_err(|_| Error::new("TODO6"))?;

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_locale_windows_constructors() {
        let _ = SystemLocale::default().unwrap();
        let _ = SystemLocale::from_name("en-US").unwrap();
        let names = SystemLocale::available_names().unwrap();
        for name in &names {
            let _ = SystemLocale::from_name(name).unwrap();            
        }
    }

    #[test]
    fn test_system_locale_windows_available_names() {
        use std::sync::mpsc;
        use std::thread;

        let locales = SystemLocale::available_names().unwrap();

        let (sender, receiver) = mpsc::channel();

        let mut handles = Vec::new();
        for _ in 0..20 {
            let sender = sender.clone();
            let handle = thread::spawn(move || {
                let locales = enum_system_locales_ex().unwrap();
                sender.send(locales).unwrap();
            });
            handles.push(handle);
        }

        let mut localess = Vec::new();
        for _ in handles {
            let locales = receiver.recv().unwrap();
            localess.push(locales);
        }

        for locales2 in localess {
            for locale2 in &locales2 {
                if !locales.contains(locale2) {
                    eprintln!("{} in locales2 but not locales1", locale2);
                }
            }
            for locale1 in &locales {
                if !locales2.contains(locale1) {
                    eprintln!("{} in locales1 but not locales2", locale1);
                }
            }
            assert_eq!(locales, locales2)
        }
    }
}
