#![cfg(windows)]

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));
}

use std::mem;
use std::ptr;

use widestring::U16CString;
use winapi::ctypes::{c_ulong, wchar_t};
use winapi::um::winnls::GetLocaleInfoEx;
use winapi::um::winnls::{GetNumberFormatEx, GetUserDefaultLocaleName, NUMBERFMTW};

use crate::{Error, Locale, SystemLocale};

impl SystemLocale {
    pub fn new() -> Result<SystemLocale, Error> {
        let name = U16CString::from_str("en-US").map_err(|e| Error::new(&e.to_string()))?;
        let s = get_locale_info_ex(&name, bindings::LOCALE_SGROUPING)?;
        println!("{}", &s);

        let locale = SystemLocale {
            dec: Locale::en.decimal(),
            grp: Locale::en.grouping(),
            inf: Locale::en.infinity().to_string(),
            min: Locale::en.minus_sign().to_string(),
            nan: Locale::en.nan().to_string(),
            sep: Locale::en.separator(),
        };

        Ok(locale)
    }
}

fn get_locale_info_ex(lp_locale_name: &U16CString, lc_type: c_ulong) -> Result<String, Error> {
    let size = unsafe { GetLocaleInfoEx(lp_locale_name.as_ptr(), lc_type, ptr::null_mut(), 0) };
    let mut buf: Vec<wchar_t> = vec![0; 1024];
    let err = unsafe { GetLocaleInfoEx(lp_locale_name.as_ptr(), lc_type, buf.as_mut_ptr(), size) };
    if err == 0 {
        return Err(Error::new("TODO: something went wrong"));
    }
    let windows_string = U16CString::from_vec_with_nul(buf).map_err(|_| Error::new("todo"))?;
    let s = windows_string.to_string().map_err(|_| Error::new("todo"))?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows() {
        let _ = SystemLocale::new().unwrap();
    }
}
