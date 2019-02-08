#![cfg(unix)]

use crate::constants::{MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN};
use crate::utils;
use crate::{Error, Locale, SystemLocale};

impl SystemLocale {
    pub fn new() -> Result<SystemLocale, Error> {
        let empty_slice = &['\0' as libc::c_char];
        let _ = unsafe { libc::setlocale(libc::LC_MONETARY, empty_slice.as_ptr()) };

        let ptr = unsafe { libc::localeconv() };
        if ptr.is_null() {
            return Err(Error::c("C function 'localeconv' returned a null pointer."));
        }

        let lconv: &libc::lconv = unsafe { ptr.as_ref() }.unwrap();

        let dec_ptr = utils::Pointer::new(lconv.mon_decimal_point)?;
        let grp_ptr = utils::Pointer::new(lconv.mon_grouping)?;
        let min_ptr = utils::Pointer::new(lconv.negative_sign)?;
        let sep_ptr = utils::Pointer::new(lconv.mon_thousands_sep)?;

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
