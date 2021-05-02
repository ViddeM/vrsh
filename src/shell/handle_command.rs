use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::process::{Child, Command, Stdio, ChildStdout};
use crate::shell::handle_command::CommandError::FailedToSpawnChild;
use std::io::{Read};
use crate::shell::built_ins::cd::handle_dir_change;
use crate::shell::built_ins::alias::handle_alias;
use crate::shell::common::types::{Redirect, Cmd, CmdPart};
use crate::shell::built_ins::errors::BuiltInError;
use crate::shell::common::state::State;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

pub enum CommandStatus {
    Ok,
    Exit,
}

pub enum CommandError {
    IO(std::io::Error),
    FailedToSpawnChild(String, std::io::Error),
    BuiltInError(BuiltInError),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::IO(e) => write!(f, "{}", e),
            CommandError::FailedToSpawnChild(cmd, e) => write!(f, "failed to spawn process for command '{}': {}", cmd, e),
            CommandError::BuiltInError(e) => write!(f, "{}", e),
        }
    }
}

impl From<BuiltInError> for CommandError {
    fn from(err: BuiltInError) -> Self { CommandError::BuiltInError(err) }
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self { CommandError::IO(err) }
}

pub fn handle_command(command: Cmd, state: &mut State) -> Result<CommandStatus, CommandError> {
    match handle_command_with_output(command, Stdio::inherit(), state) {
        Ok((status, _)) => {
            // Somewhat ugly hack to make sure we always get a newline after a command.
            let mut stdout = std::io::stdout().into_raw_mode()?;
            let (x, _) = stdout.cursor_pos()?;
            if x != 1 {
                // TODO: do something more fun with this, e.g. print in red or smth
                print!("\t%vrsh: missing newline%\n")
            }
            Ok(status)
        },
        Err(e) => Err(e)
    }
}

fn handle_command_with_output(command: Cmd, output: Stdio, state: &mut State) -> Result<(CommandStatus, Option<ChildStdout>), CommandError> {
    let mut all_prevs: Vec<Child> = Vec::new();
    let mut output = Some(output);
    for (index, part) in command.parts.into_iter().enumerate().rev() {
        match part.cmd.as_str() {
            "exit" => return Ok((CommandStatus::Exit, None)),
            "cd" => match handle_dir_change(part.args) {
                Ok(_) => {}
                Err(e) => println!("vrsh: {}", e)
            },
            "alias" => match handle_alias(part.args, state) {
                Ok(_) => {}
                Err(e) => println!("vrsh: {}", e)
            }
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
    let val = File::create(file)?;
    Ok(Stdio::from(val))
}

fn open_redirect_file(file: String) -> Result<Stdio, CommandError> {
    let val = File::open(file)?;
    Ok(Stdio::from(val))
}

fn run_command(part: CmdPart, output: Stdio, input: Stdio) -> Result<Child, CommandError> {
    return match Command::new(&part.cmd)
        .args(part.args.iter().map(|v| v.to_string()).collect::<Vec<String>>())
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

pub fn handle_sub_command(command: Cmd, state: &mut State) -> Result<String, CommandError> {
    match handle_command_with_output(command, Stdio::piped(), state) {
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

