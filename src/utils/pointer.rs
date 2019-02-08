#![cfg(unix)]

use core::marker::PhantomData;
use core::slice;
use core::str;

use libc::c_char;

use crate::constants::MAX_MIN_LEN;
use crate::{Error, Grouping};

pub(crate) struct Pointer<'a> {
    ptr: *const c_char,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Pointer<'a> {
    pub(crate) fn new(ptr: *const c_char) -> Result<Pointer<'a>, Error> {
        if ptr.is_null() {
            return Err(Error::c("received a null pointer from C."));
        }
        Ok(Pointer {
            ptr,
            phantom: PhantomData,
        })
    }

    pub(crate) fn as_char(&self) -> Result<Option<char>, Error> {
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

    pub(crate) fn as_grouping(&self) -> Result<Grouping, Error> {
        let len = unsafe { libc::strlen(self.ptr) };
        let s = unsafe { slice::from_raw_parts(self.ptr as *const u8, len) };
        match s {
            [3, 3] => Ok(Grouping::Standard),
            [3, 2] => Ok(Grouping::Indian),
            [] => Ok(Grouping::Posix),
            _ => Err(Error::c("received unexpected grouping code from C")),
        }
    }

    pub(crate) fn as_str(&self) -> Result<&str, Error> {
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
