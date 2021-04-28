use std::env::set_current_dir;
use std::path::Path;
use crate::shell::common::types::Arg;
use crate::shell::built_ins::errors::BuiltInError;

pub fn handle_dir_change(args: Vec<Arg>) -> Result<(), BuiltInError> {
    return match args.len() {
        0 => Err(BuiltInError::NoArgument),
        1 => match args.first() {
            Some(arg) => match set_current_dir(&Path::new(match arg {
                Arg::Word(a) => a.as_str(),
                _ => return Err(BuiltInError::InvalidArgument)
            })) {
                Err(e) => Err(BuiltInError::FailedToChangeDir(e)),
                _ => Ok(())
            },
            None => Err(BuiltInError::FailedToExtractArg),
        },
        num => Err(BuiltInError::TooManyArguments(num, 1)),
    }
}