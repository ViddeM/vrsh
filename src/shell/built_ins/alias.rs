use crate::shell::common::types::Arg;
use crate::shell::built_ins::errors::BuiltInError;

pub fn handle_alias(args: Vec<Arg>) -> Result<String, BuiltInError> {
    println!("Received args: {:?}", args);

    match args.len() {
        0 => return Err(BuiltInError::NoArgument),
        1 => {},
        _ => return Err(BuiltInError::TooManyArguments(args.len(), 1)),
    }

    let arg = args[0].to_owned();

    println!("{:?}", arg);
    // match arg {
    //     Arg::Word(_) => Err()
    //     Arg::String(_) => {}
    // }

    Ok("".to_string())
}