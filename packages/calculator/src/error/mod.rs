use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseError,
    NullError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError => write!(f, "ParseError"),
            Error::NullError => write!(f, "NullError"),
        }
    }
}
