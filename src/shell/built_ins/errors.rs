use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error;

pub enum BuiltInError {
    NoArgument,
    FailedToExtractArg,
    TooManyArguments(usize, usize),
    FailedToChangeDir(std::io::Error),
    FailedToSpawnChild(String, std::io::Error),
    InvalidArgument,
    IOError(std::io::Error),
    NoSuchProgram(String),
}

impl Display for BuiltInError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInError::NoArgument => write!(f, "no argument provided"),
            BuiltInError::FailedToExtractArg => write!(f, "failed to extract argument"),
            BuiltInError::TooManyArguments(got, expected) => {
                write!(f, "too many arguments, got {}, expected {}", got, expected)
            }
            BuiltInError::FailedToChangeDir(e) => write!(f, "failed to change dir: {}", e),
            BuiltInError::InvalidArgument => write!(f, "invalid argument"),
            BuiltInError::IOError(e) => write!(f, "ioerror: {}", e),
            BuiltInError::FailedToSpawnChild(cmd, e) => {
                write!(f, "failed to spawn child for command {}: {}", cmd, e)
            }
            BuiltInError::NoSuchProgram(program) => write!(f, "no such program {}", program),
        }
    }
}

impl From<std::io::Error> for BuiltInError {
    fn from(e: Error) -> Self {
        BuiltInError::IOError(e)
    }
}
