// http://www.cplusplus.com/reference/clocale/lconv/
// https://lh.2xlibre.net/

use std::ffi::{CStr, CString};

use lazy_static::lazy_static;
use libc;

use crate::details::Grouping;
use crate::error::Error;

lazy_static! {
    pub(crate) static ref ENVIRONMENT: Option<Environment> = unsafe { Environment::init().ok() };
}

/// Struct representing environment format settings (i.e. those from `LC_ALL`, `LC_NUMERIC`, etc.). Implements `Format`.
///
/// # Note
///
/// The first time `Environment::new()` is called, it calls into C in order to get format settings
/// from environment variables (e.g. `LC_ALL`, `LC_NUMERIC`, etc.) as one would do in a C program.
/// More specifically, it calls `setlocale` and `localeconv` from `locale.h`. This can fail, which
/// is why it's the only function/method in this crate which can return an `Error`. Any subsequent
/// calls to `Environment::new()` just return a globally chached reference to the information that
/// was obtained from the initial call.
///
/// The information we receive back from C does not specify all possible formatting options.
/// In particular, it's missing information on how to represent "infinity", "nan", and "percent
/// sign"; so when you use it, these options will be set to the default ("∞", "NaN", and "%",
/// respectively).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Environment {
    decimal: char,
    grouping: String,
    minus_sign: char,
    separator: char,
}

impl Environment {
    pub fn new() -> Result<&'static Environment, Error> {
        Ok((&*ENVIRONMENT).as_ref().ok_or_else(|| Error::C)?)
    }

    unsafe fn init() -> Result<Environment, Error> {
        let empty = CString::new("").unwrap();
        let ptr = empty.as_c_str().as_ptr();
        libc::setlocale(libc::LC_ALL, ptr);
        let ptr = libc::localeconv();
        let lconv = ptr.as_ref().ok_or_else(|| Error::C)?;

        // decimal
        let ptr = lconv.decimal_point;
        if ptr.is_null() {
            return Err(Error::C);
        }
        let decimal = CStr::from_ptr(ptr)
            .to_str()
            .map_err(|_| Error::C)?
            .to_string();

        // grouping
        let ptr = lconv.grouping;
        if ptr.is_null() {
            return Err(Error::C);
        }
        let grouping = CStr::from_ptr(ptr)
            .to_str()
            .map_err(|_| Error::C)?
            .to_string();

        // minus_sign
        let ptr = lconv.negative_sign;
        if ptr.is_null() {
            return Err(Error::C);
        }
        let minus_sign = CStr::from_ptr(ptr)
            .to_str()
            .map_err(|_| Error::C)?
            .to_string();

        // separator
        let ptr = lconv.thousands_sep;
        if ptr.is_null() {
            return Err(Error::C);
        }
        let separator = CStr::from_ptr(ptr)
            .to_str()
            .map_err(|_| Error::C)?
            .to_string();

        unimplemented!()
    }

    pub fn decimal(&self) -> char {
        self.decimal
    }

    pub fn grouping(&self) -> Grouping {
        unimplemented!()
    }

    pub fn minus_sign(&self) -> char {
        self.minus_sign
    }

    pub fn separator(&self) -> char {
        self.separator
    }
}
