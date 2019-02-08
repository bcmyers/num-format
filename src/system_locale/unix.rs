#![cfg(unix)]

use std::marker::PhantomData;
use std::slice;
use std::str;

use libc::c_char;

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN};
use crate::{Error, Locale, SystemLocale, Grouping};

impl SystemLocale {
    pub fn new() -> Result<SystemLocale, Error> {
        let empty_slice = &['\0' as libc::c_char];
        let _ = unsafe { libc::setlocale(libc::LC_MONETARY, empty_slice.as_ptr()) };

        let ptr = unsafe { libc::localeconv() };
        if ptr.is_null() {
            return Err(Error::c("C function 'localeconv' returned a null pointer."));
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

        let locale = SystemLocale {
            dec: maybe_dec.unwrap_or_else(|| '.'),
            grp,
            inf: Locale::en.infinity().to_string(),
            min: min.to_string(),
            nan: Locale::en.nan().to_string(),
            sep: maybe_sep,
        };

        Ok(locale)
    }

    pub fn set_infinity<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        let s = s.into();
        if s.len() > MAX_INF_LEN {
            return Err(Error::new("TODO"));
        }
        self.nan = s;
        Ok(())
    }

    pub fn set_nan<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        let s = s.into();
        if s.len() > MAX_NAN_LEN {
            return Err(Error::new("TODO"));
        }
        self.nan = s;
        Ok(())
    }
}

struct Pointer<'a> {
    ptr: *const c_char,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Pointer<'a> {
    fn new(ptr: *const c_char) -> Result<Pointer<'a>, Error> {
        if ptr.is_null() {
            return Err(Error::c("received a null pointer from C."));
        }
        Ok(Pointer {
            ptr,
            phantom: PhantomData,
        })
    }

    fn as_char(&self) -> Result<Option<char>, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        let s = str::from_utf8(s)
            .map_err(|_| Error::c("could not parse data returned from C into utf-8"))?;
        if s.chars().count() > 1 {
            return Err(Error::c(
                "received C string of length greater than 1 when C string of length 1 was expected",
            ));
        }
        Ok(s.chars().next())
    }

    fn as_grouping(&self) -> Result<Grouping, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        match s {
            [3, 3] => Ok(Grouping::Standard),
            [3, 2] => Ok(Grouping::Indian),
            [] => Ok(Grouping::Posix),
            _ => Err(Error::c("received unexpected grouping code from C")),
        }
    }

    fn as_str(&self) -> Result<&str, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        let s = str::from_utf8(s)
            .map_err(|_| Error::c("could not parse data returned from C into utf-8"))?;
        if s.len() > MAX_MIN_LEN {
            return Err(Error::capacity(s.len(), MAX_MIN_LEN));
        }
        Ok(s)
    }
}
