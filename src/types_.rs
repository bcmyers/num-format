#![allow(dead_code)] // TODO

use std::ops::Deref;

use arrayvec::ArrayString;

use crate::constants::{MAX_DEC_LEN, MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN, MAX_SEP_LEN};
use crate::error::Error;

macro_rules! create_type {
    ( $name:ident, $max_len:expr ) => {
        /// TODO
        #[derive(Clone, Debug)]
        pub(crate) struct $name(ArrayString<[u8; MAX_DEC_LEN]>);

        impl $name {
            pub(crate) fn new<S>(s: S) -> Result<Self, Error> where S: AsRef<str> {
                let s = s.as_ref();
                let a = ArrayString::from(s).map_err(|_| Error::capacity(s.len(), $max_len))?;
                Ok($name(a))
            }

            pub(crate) fn as_str(&self) -> &str {
                self
            }
        }

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &str {
                self.0.deref()
            }
        }
    }
}

create_type!(Dec, MAX_DEC_LEN);
create_type!(Inf, MAX_INF_LEN);
create_type!(Min, MAX_MIN_LEN);
create_type!(Nan, MAX_NAN_LEN);
create_type!(Sep, MAX_SEP_LEN);
