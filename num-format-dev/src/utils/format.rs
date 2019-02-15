use crate::utils::Grouping;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Format {
    pub(crate) identifier: String,

    pub(crate) dec: char,
    pub(crate) grp: Grouping,
    pub(crate) inf: String,
    pub(crate) min: String,
    pub(crate) nan: String,
    pub(crate) pos: String,
    pub(crate) sep: char,
}
