#[derive(Debug, PartialEq)]
pub enum Error {
    ParsingFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParsingFailed => write!(f, "failed to parse input"),
        }
    }
}
