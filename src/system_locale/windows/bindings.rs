#![cfg(windows)]

mod raw {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));
}

use lazy_static::lazy_static;
use winapi::shared::minwindef::DWORD;

lazy_static! {
    pub(crate) static ref LOCALE_NAME_SYSTEM_DEFAULT: &'static str = {
        let raw = raw::LOCALE_NAME_SYSTEM_DEFAULT;
        unsafe { std::str::from_utf8_unchecked(&raw[0..raw.len() - 1]) }
    };
}

pub(crate) const LOCALE_NAME_MAX_LENGTH: usize = raw::LOCALE_NAME_MAX_LENGTH as usize;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Request {
    Decimal,
    Grouping,
    MinusSign,
    Nan,
    NegativeInfinity,
    PositiveInfinity,
    Separator,
}

impl From<Request> for DWORD {
    fn from(request: Request) -> DWORD {
        match request {
            Request::Decimal => raw::LOCALE_SDECIMAL,
            Request::Grouping => raw::LOCALE_SGROUPING,
            Request::MinusSign => raw::LOCALE_SNEGATIVESIGN,
            Request::Nan => raw::LOCALE_SNAN,
            Request::NegativeInfinity => raw::LOCALE_SNEGINFINITY,
            Request::PositiveInfinity => raw::LOCALE_SPOSINFINITY,
            Request::Separator => raw::LOCALE_STHOUSAND,
        }
    }
}