use crate::shell::built_ins::errors::BuiltInError;
use crate::shell::common::types::{CmdPart, Redirect};
use std::fs::File;
use std::process::{Child, Command, Stdio};

pub fn execute_command(
    part: CmdPart,
    output: Option<Stdio>,
    index: usize,
) -> Result<Child, BuiltInError> {
    let mut redirect_in: Option<String> = None;
    let mut redirect_out: Option<String> = None;
    for redirect in part.redirects.iter() {
        match redirect {
            Redirect::In(val) => redirect_in = Some(val.clone()),
            Redirect::Out(val) => redirect_out = Some(val.clone()),
        }
    }

    let cmd_out = if let Some(file) = redirect_out {
        create_redirect_file(file)?
    } else {
        match output {
            None => Stdio::piped(),
            Some(out) => out,
        }
    };

    let cmd_in = if let Some(file) = redirect_in {
        open_redirect_file(file)?
    } else if index == 0 {
        Stdio::inherit()
    } else {
        Stdio::piped()
    };

    return run_command(part, cmd_out, cmd_in);
}

fn create_redirect_file(file: String) -> Result<Stdio, BuiltInError> {
    let val = File::create(file)?;
    Ok(Stdio::from(val))
}

fn open_redirect_file(file: String) -> Result<Stdio, BuiltInError> {
    let val = File::open(file)?;
    Ok(Stdio::from(val))
}

fn run_command(part: CmdPart, output: Stdio, input: Stdio) -> Result<Child, BuiltInError> {
    return match Command::new(&part.cmd)
        .args(
            part.args
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        )
        .stdout(output)
        .stdin(input)
        .spawn()
    {
        Ok(c) => Ok(c),
        Err(e) => Err(BuiltInError::FailedToSpawnChild(part.cmd, e)),
    };
}
