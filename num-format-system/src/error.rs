/// TODO
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Error;

impl Error {
    /// TODO
    pub fn new(_: &str) -> Error {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub(crate) fn capacity(_: usize, _: usize) -> Error {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub(crate) fn unix(_: &str) -> Error {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub(crate) fn windows<S>(_: S) -> Error where S: AsRef<str> {
        unimplemented!()
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Error {
        unimplemented!()
    }
}

#[cfg(windows)]
impl From<widestring::NulError<u16>> for Error {
    fn from(_: widestring::NulError<u16>) -> Error {
        unimplemented!()
    }
}
