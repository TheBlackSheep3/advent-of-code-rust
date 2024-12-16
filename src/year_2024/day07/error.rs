#[derive(Debug, PartialEq)]
pub enum Error {
    ParsingFailed,
    IntegerTypeTooSmall,
    BitFieldGeneration,
    EnumerationFieldGeneration,
    AccumulationFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParsingFailed => write!(f, "failed to parse input"),
            Error::IntegerTypeTooSmall => {
                write!(f, "failed to parse integer because data type is too small")
            }
            Error::BitFieldGeneration => write!(
                f,
                "failed to generate proper bit field from unsinged integer"
            ),
            Error::EnumerationFieldGeneration => write!(
                f,
                "failed to generate proper enumeration field from unsinged integer"
            ),
            Error::AccumulationFailed => write!(f, "result accumulation failed"),
        }
    }
}
