use crate::shell::command::Cmd;
use std::env::{current_dir, var_os};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Error, Write};

pub enum ParseError {
    IO(std::io::Error),
    NoCommand,
    NoWorkingDir,
    NoHomeVar,
    Internal
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IO(e) => write!(f, "io error: '{}'", e),
            ParseError::NoCommand => write!(f, "no command provided"),
            ParseError::NoWorkingDir => write!(f, "failed to retrieve current working directory"),
            ParseError::NoHomeVar => write!(f, "environment variable HOME not set or set to empty"),
            ParseError::Internal => write!(f, "something went wrong"),
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: Error) -> Self {
        ParseError::IO(err)
    }
}

pub fn parse_input() -> Result<Cmd, ParseError> {
    let prompt = get_prompt()?;

    print!("{} > ", prompt);
    // Make sure it is printed immediately
    stdout().flush()?;

    let mut s = String::new();
    stdin().read_line(&mut s)?;
    let command = s
        .trim()
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>();
    let cmd_args = match command.split_first() {
        Some(val) => val,
        None => return Err(ParseError::NoCommand),
    };

    return Ok(Cmd {
        cmd: cmd_args.0.to_string(),
        args: cmd_args
            .1
            .into_iter()
            .map(String::to_string)
            .collect::<Vec<String>>(),
    });
}

fn get_prompt() -> Result<String, ParseError> {
    let curr_dir = current_dir()?;
    let wd = match curr_dir.to_str() {
        Some(dir) => dir,
        None => return Err(ParseError::NoWorkingDir),
    };

    let home_dir = get_home_dir()?;

    let prompt = wd.replace(home_dir.as_str(), "~");
    return Ok(prompt)
}

fn get_home_dir() -> Result<String, ParseError> {
    return match var_os("HOME") {
        Some(os_s) => {
            if os_s.is_empty() {
                return Err(ParseError::NoHomeVar)
            }
            match os_s.to_str() {
                None => Err(ParseError::Internal),
                Some(s) => Ok(s.to_string())
            }
        }
        None => Err(ParseError::NoHomeVar)
    };
}