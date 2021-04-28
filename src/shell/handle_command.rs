use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::process::{Child, Command, Stdio, ChildStdout};
use crate::shell::handle_command::CommandError::FailedToSpawnChild;
use std::io::{Read};
use crate::shell::built_ins::cd::handle_dir_change;
use crate::shell::built_ins::alias::handle_alias;
use crate::shell::common::types::{Redirect, Cmd, CmdPart, Arg};

pub enum CommandStatus {
    Ok,
    Exit,
}

pub enum CommandError {
    IO(std::io::Error),
    FailedToSpawnChild(String, std::io::Error),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::IO(e) => write!(f, "{}", e),
            CommandError::FailedToSpawnChild(cmd, e) => write!(f, "failed to spawn process for command '{}': {}", cmd, e),
        }
    }
}

pub fn handle_command(command: Cmd) -> Result<CommandStatus, CommandError> {
    match handle_command_with_output(command, Stdio::inherit()) {
        Ok((status, _)) => Ok(status),
        Err(e) => Err(e)
    }
}

fn handle_command_with_output(command: Cmd, output: Stdio) -> Result<(CommandStatus, Option<ChildStdout>), CommandError> {
    let mut all_prevs: Vec<Child> = Vec::new();
    let mut output = Some(output);
    for (index, part) in command.parts.into_iter().enumerate().rev() {
        match part.cmd.as_str() {
            "exit" => return Ok((CommandStatus::Exit, None)),
            "cd" => match handle_dir_change(part.args) {
                Ok(_) => {}
                Err(e) => println!("vrsh: {}", e)
            },
            "alias" => handle_alias(part.args),
            _ => {
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
                        Some(out) => out
                    }
                };

                let cmd_in = if let Some(file) = redirect_in {
                    open_redirect_file(file)?
                } else if index == 0 {
                    Stdio::inherit()
                } else {
                    Stdio::piped()
                };

                match run_command(part, cmd_out, cmd_in) {
                    Ok(mut c) => {
                        output = Some(match c.stdin.take() {
                            Some(v) => Stdio::from(v),
                            None => Stdio::inherit(),
                        });

                        all_prevs.push(c);
                    }
                    Err(e) => return Err(e)
                }
            }
        }
    }

    let mut res = None;
    for (index, mut child) in all_prevs.into_iter().enumerate() {
        if index == 0 {
            res = child.stdout.take()
        }

        match child.wait() {
            Ok(_) => {}
            Err(e) => println!("Failed to wait for child {}", e),
        }
    }

    Ok((CommandStatus::Ok, res))
}

fn create_redirect_file(file: String) -> Result<Stdio, CommandError> {
    match File::create(file) {
        Ok(val) => Ok(Stdio::from(val)),
        Err(e) => Err(CommandError::IO(e)),
    }
}

fn open_redirect_file(file: String) -> Result<Stdio, CommandError> {
    match File::open(file) {
        Ok(val) => Ok(Stdio::from(val)),
        Err(e) => Err(CommandError::IO(e)),
    }
}

fn run_command(part: CmdPart, output: Stdio, input: Stdio) -> Result<Child, CommandError> {
    return match Command::new(&part.cmd)
        .args(part.args.iter().map(|v| match v {
            Arg::Word(s) => Ok(s.to_owned()),
            Arg::String(s) => Ok(s.to_owned()),
        }).collect::<Result<Vec<String>, CommandError>>()?)
        .stdout(output)
        .stdin(input)
        .spawn()
    {
        Ok(c) => Ok(c),
        Err(e) => {
            Err(FailedToSpawnChild(part.cmd, e))
        }
    };
}

pub fn handle_sub_command(command: Cmd) -> Result<String, CommandError> {
    match handle_command_with_output(command, Stdio::piped()) {
        Ok((_, child)) => match child {
            None => Ok("".to_string()),
            Some(mut c) => {
                let mut buffer = String::new();
                match c.read_to_string(&mut buffer) {
                    Ok(_) => {
                        Ok(buffer.replace("\n", " "))
                    },
                    Err(_) => Ok("".to_string())
                }
            }
        }
        Err(e) => Err(e)
    }
}

