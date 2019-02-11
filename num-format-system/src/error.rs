use std::fmt;

/// TODO
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Error {
    message: String,
}

impl Error {
    /// TODO
    pub fn new(msg: &str) -> Error {
        Error {
            message: format!("TODO: other: {}", msg),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn capacity(_: usize, _: usize) -> Error {
        Error {
            message: format!("TODO: Capacity error"),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn unix(msg: &str) -> Error {
        Error {
            message: format!("TODO: unix: {}", msg),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn windows<S>(msg: S) -> Error
    where
        S: AsRef<str>,
    {
        Error {
            message: format!("TODO: windows: {}", msg.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Error {
        panic!("TODO: std::ffi::NulError")
    }
}

#[cfg(windows)]
impl From<widestring::NulError<u16>> for Error {
    fn from(_: widestring::NulError<u16>) -> Error {
        panic!("TODO: widestring::NulError<u16>>")
    }
}
