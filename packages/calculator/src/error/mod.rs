use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotDigit,
    EmptyToken,
    NotMatchToken,
    NotSupportOP,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyToken => write!(f, "EmptyToken"),
            Error::NotDigit => write!(f, "NotDigit"),
            Error::NotMatchToken => write!(f, "NotMatchToken"),
            Error::NotSupportOP => write!(f, "NotSupportOP"),
        }
    }
}
