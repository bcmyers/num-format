#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Number {
    Integer { s: String, is_positive: bool },
    FloatNormal { s: String, is_positive: bool },
    FloatInfinity { is_positive: bool },
    FloatNan,
}
