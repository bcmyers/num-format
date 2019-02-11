/// TODO
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Error;

impl Error {
    /// TODO
    pub fn new(msg: &str) -> Error {
        panic!("TODO: other: {}", msg)
    }

    #[allow(dead_code)]
    pub(crate) fn capacity(_: usize, _: usize) -> Error {
        panic!("TODO: Capacity error")
    }

    #[allow(dead_code)]
    pub(crate) fn unix(msg: &str) -> Error {
        panic!("TODO: unix: {}", msg)
    }

    #[allow(dead_code)]
    pub(crate) fn windows<S>(msg: S) -> Error
    where
        S: AsRef<str>,
    {
        panic!("TODO: windows: {}", msg.as_ref())
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
