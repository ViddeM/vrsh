use crate::shell::command::{Cmd};
use std::env::{current_dir, var_os};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Error};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::shell::rl_helper::RLHelper;

use crate::grammar::CommandParser;

pub enum ParseError {
    IO(std::io::Error),
    NoWorkingDir,
    NoHomeVar,
    Internal,
    RLError(ReadlineError),
    RLIgnore,
    LALRPopErr(String)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IO(e) => write!(f, "io error: '{}'", e),
            ParseError::NoWorkingDir => write!(f, "failed to retrieve current working directory"),
            ParseError::NoHomeVar => write!(f, "environment variable HOME not set or set to empty"),
            ParseError::Internal => write!(f, "something went wrong"),
            ParseError::RLError(e) => write!(f, "readline encountered an error: {}", e),
            ParseError::LALRPopErr(s) => write!(f, "failed parsing: {}", s),
            ParseError::RLIgnore => write!(f, "ignored error"),
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: Error) -> Self {
        ParseError::IO(err)
    }
}

impl From<ReadlineError> for ParseError {
    fn from(rle: ReadlineError) -> Self {
        ParseError::RLError(rle)
    }
}

// TODO: extend grammar so that we can replace all ~ to /home/user automatically.
const HOME: &str = "~";

pub fn parse_input(rl: &mut Editor<RLHelper>) -> Result<Cmd, ParseError> {
    let prompt_prefix = get_prompt()?;
    let prompt = format!("{} > ", prompt_prefix);

    let s = match rl.readline(prompt.as_str()) {
        Ok(val) => val,
        Err(e) => {
            return match e {
                ReadlineError::Interrupted => Err(ParseError::RLIgnore),
                _ => Err(ParseError::RLError(e)),
            }
        }
    };

    let command: Cmd = match CommandParser::new().parse(&s) {
        Ok(val) => val,
        Err(e) => return Err(ParseError::LALRPopErr(e.to_string())),
    };

    rl.add_history_entry(s);
    return Ok(command);
}

fn get_prompt() -> Result<String, ParseError> {
    let curr_dir = current_dir()?;
    let wd = match curr_dir.to_str() {
        Some(dir) => dir,
        None => return Err(ParseError::NoWorkingDir),
    };

    let home_dir = get_home_dir()?;
    let prompt = wd.replace(home_dir.as_str(), HOME);

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