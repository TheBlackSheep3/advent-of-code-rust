use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParsingFailed,
    IntegerConversionFailed,
    IntegerOverflow,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsingFailed => write!(f, "failed to parse map"),
            Self::IntegerConversionFailed => write!(f, "failed to convert integer"),
            Self::IntegerOverflow => write!(f, "an integer overflow occurred"),
        }
    }
}
