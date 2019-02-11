/// TODO
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Error;

impl Error {
    /// TODO
    pub fn new(_: &str) -> Error {
        unimplemented!()
    }

    pub(crate) fn capacity(_: usize, _: usize) -> Error {
        unimplemented!()
    }

    pub(crate) fn unix(_: &str) -> Error {
        unimplemented!()
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Error {
        unimplemented!()
    }
}
