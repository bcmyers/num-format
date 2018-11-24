use failure::Fail;

/// This crate's error type. Implements `Fail`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Fail)]
pub enum Error {
    /// failed to load environment from C
    #[fail(display = "failed to load environment from C")]
    C,
}
