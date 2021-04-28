use std::fmt::{Formatter, Display};
use std::fmt;

pub enum BuiltInError {
    NoArgument,
    FailedToExtractArg,
    TooManyArguments(usize, usize),
    FailedToChangeDir(std::io::Error),
    InvalidArgument
}

impl Display for BuiltInError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInError::NoArgument => write!(f, "no argument provided"),
            BuiltInError::FailedToExtractArg => write!(f, "failed to extract argument"),
            BuiltInError::TooManyArguments(got, expected) => write!(f, "too many arguments, got {}, expected {}", got, expected),
            BuiltInError::FailedToChangeDir(e) => write!(f, "failed to change dir: {}", e),
            BuiltInError::InvalidArgument => write!(f, "invalid argument")
        }
    }
}