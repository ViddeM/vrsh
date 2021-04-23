use crate::shell::command::{Arg, Cmd, CmdPart, Redirect};
use std::env::set_current_dir;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub enum CommandStatus {
    Ok,
    Exit,
}

pub enum CommandError {
    IO(std::io::Error),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::IO(e) => write!(f, "{}", e),
        }
    }
}

pub fn handle_command(command: Cmd) -> Result<CommandStatus, CommandError> {
    let mut all_prevs: Vec<Child> = Vec::new();
    let mut output = Some(Stdio::inherit());
    for (index, part) in command.parts.into_iter().enumerate().rev() {
        match part.cmd.as_str() {
            "exit" => return Ok(CommandStatus::Exit),
            "cd" => handle_dir_change(part.args),
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
                        Some(v) => {
                            output = None;
                            v
                        }
                    }
                };

                let cmd_in = if let Some(file) = redirect_in {
                    open_redirect_file(file)?
                } else if index == 0 {
                    Stdio::inherit()
                } else {
                    Stdio::piped()
                };

                if let Some(mut c) = run_command(part, cmd_out, cmd_in) {
                    output = Some(match c.stdin.take() {
                        Some(v) => Stdio::from(v),
                        None => Stdio::inherit(),
                    });

                    all_prevs.push(c);
                }
            }
        }
    }

    for mut child in all_prevs.into_iter() {
        match child.wait() {
            Ok(_) => {}
            Err(e) => println!("Failed to wait for child {}", e),
        }
    }

    Ok(CommandStatus::Ok)
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

fn run_command(part: CmdPart, output: Stdio, input: Stdio) -> Option<Child> {
    return match Command::new(&part.cmd)
        .args(part.args.iter().map(|v| v.word.as_str()))
        .stdout(output)
        .stdin(input)
        .spawn()
    {
        Ok(c) => Some(c),
        Err(e) => {
            println!("Failed to spawn process {}", e);
            None
        }
    };
}

fn handle_dir_change(args: Vec<Arg>) {
    match args.len() {
        0 => println!("must provide an argument"),
        1 => match args.first() {
            Some(arg) => match set_current_dir(&Path::new(arg.word.as_str())) {
                Err(e) => println!("failed to change working dir {}", e),
                _ => {}
            },
            None => println!("failed to handle args"),
        },
        num => println!("invalid amount of arguments to cd: {}", num),
    }
}
