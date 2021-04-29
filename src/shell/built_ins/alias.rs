use crate::shell::common::types::{Arg, Assignment};
use crate::shell::built_ins::errors::BuiltInError;
use crate::shell::common::state::State;

pub fn handle_alias(args: Vec<Arg>, state: &mut State) -> Result<(), BuiltInError> {
    match args.len() {
        0 => return Err(BuiltInError::NoArgument),
        1 => {},
        _ => return Err(BuiltInError::TooManyArguments(args.len(), 1)),
    }

    let arg = args[0].to_owned();
    match arg {
        Arg::Assignment(w, a) => {
            state.aliases.insert(w, match a {
                Assignment::Word(w) => w,
                Assignment::String(s) => s
            });
        }
        _ => return Err(BuiltInError::InvalidArgument)
    }

    Ok(())
}