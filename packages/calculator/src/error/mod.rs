use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseToDigitError,
    NullTokenError,
    TokenTypeMatchError,
    DestructError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NullTokenError => write!(f, "NullTokenError"),
            Error::ParseToDigitError => write!(f, "ParseToDigitError"),
            Error::TokenTypeMatchError => write!(f, "TokenTypeMatchError"),
            Error::DestructError => write!(f, "DestructError"),
        }
    }
}
