use crate::shell::built_ins::alias::handle_alias;
use crate::shell::built_ins::cd::handle_dir_change;
use crate::shell::built_ins::errors::BuiltInError;
use crate::shell::built_ins::execute_command::execute_command;
use crate::shell::built_ins::set_variable::set_variable;
use crate::shell::common::colors::{bg_color, fg_color, reset_color, test_colors, Color};
use crate::shell::common::state::State;
use crate::shell::common::types::{Cmd, CmdPart, CmdType, Redirect};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::process::{Child, ChildStdout, Command, Stdio};
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

pub enum CommandStatus {
    Ok,
    Exit,
}

pub enum CommandError {
    IO(std::io::Error),
    BuiltInError(BuiltInError),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::IO(e) => write!(f, "{}", e),
            CommandError::BuiltInError(e) => write!(f, "{}", e),
        }
    }
}

impl From<BuiltInError> for CommandError {
    fn from(err: BuiltInError) -> Self {
        CommandError::BuiltInError(err)
    }
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        CommandError::IO(err)
    }
}

pub fn handle_command(command: Cmd, state: &mut State) -> Result<CommandStatus, CommandError> {
    match handle_command_with_output(command, Stdio::inherit(), state) {
        Ok((status, _)) => {
            // Somewhat ugly hack to make sure we always get a newline after a command.
            let mut stdout = std::io::stdout().into_raw_mode()?;
            let (x, _) = stdout.cursor_pos()?;
            if x != 1 {
                print!(
                    "\t{}{}%{}\n",
                    bg_color(Color::Red),
                    fg_color(Color::White),
                    reset_color()
                )
            }
            Ok(status)
        }
        Err(e) => Err(e),
    }
}

fn handle_command_with_output(
    command: Cmd,
    output: Stdio,
    state: &mut State,
) -> Result<(CommandStatus, Option<ChildStdout>), CommandError> {
    let mut all_prevs: Vec<Child> = Vec::new();
    let mut output = Some(output);
    for (index, part) in command.parts.into_iter().enumerate().rev() {
        match part {
            CmdType::Cmd(c) => match c.cmd.as_str() {
                "exit" => return Ok((CommandStatus::Exit, None)),
                "cd" => match handle_dir_change(c.args) {
                    Ok(_) => {}
                    Err(e) => println!("vrsh: {}", e),
                },
                "alias" => match handle_alias(c.args, state) {
                    Ok(_) => {}
                    Err(e) => println!("vrsh: {}", e),
                },
                "vrsh-colors" => {
                    println!("--------");
                    test_colors();
                    println!("--------");
                }
                _ => {
                    let mut c = execute_command(c, output, index)?;
                    output = Some(match c.stdin.take() {
                        Some(v) => Stdio::from(v),
                        None => Stdio::inherit(),
                    });

                    all_prevs.push(c);
                }
            },
            CmdType::Variable(var, val) => set_variable(var, val, state),
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

pub fn handle_sub_command(command: Cmd, state: &mut State) -> Result<String, CommandError> {
    match handle_command_with_output(command, Stdio::piped(), state) {
        Ok((_, child)) => match child {
            None => Ok("".to_string()),
            Some(mut c) => {
                let mut buffer = String::new();
                match c.read_to_string(&mut buffer) {
                    Ok(_) => {
                        if let Some(last) = buffer.chars().last() {
                            if last == '\n' {
                                buffer.pop();
                            }
                        }
                        Ok(buffer.replace("\n", " "))
                    }
                    Err(_) => Ok("".to_string()),
                }
            }
        },
        Err(e) => Err(e),
    }
}
