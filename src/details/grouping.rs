#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Grouping {
    /// Digits are separated into groups of 3 (e.g. 10,000,000)
    Standard,
    /// The first 3 digits are grouped together and all digits after than are
    /// separated into groups of 2 (e.g. 1,00,00,000)
    Indian,
    /// No grouping (e.g. 10000000)
    Posix,
}
