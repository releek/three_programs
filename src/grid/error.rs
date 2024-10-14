use std::fmt;

#[derive(Debug)]
pub enum Error {
    NoValues,
    ElementSumMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoValues => write!(f, "No values were provided."),
            Error::ElementSumMismatch => write!(f, "The element sums did not match."),
        }
    }
}
