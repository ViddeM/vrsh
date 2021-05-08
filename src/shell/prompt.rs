use crate::shell::common::state::State;
use std::env::current_dir;
use crate::shell::rl_helper::RLHelper;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::shell::parse_command::{parse_initial_cmd, HOME};
use std::io::Error;

pub enum ReadError {
    Ignore,
    RLError(ReadlineError),
    NoWorkingDir,
    IO(std::io::Error),
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::IO(e) => write!(f, "io error: '{}'", e),
            ReadError::NoWorkingDir => write!(f, "failed to retrieve current working directory"),
            ReadError::RLError(e) => write!(f, "readline encountered an error: {}", e),
            ReadError::Ignore => write!(f, "ignored error"),
        }
    }
}

impl From<std::io::Error> for ReadError {
    fn from(err: Error) -> Self {
        ReadError::IO(err)
    }
}

pub fn read_input(rl: &mut Editor<RLHelper>, state: &mut State) -> Result<String, ReadError> {
    let prompt = get_prompt(state)?;
    let input = match rl.readline(prompt.as_str()) {
        Ok(val) => val,
        Err(e) => return match e {
            ReadlineError::Interrupted => Err(ReadError::Ignore),
            ReadlineError::Eof => Err(ReadError::Ignore),
            _ => Err(ReadError::RLError(e))
        }
    };
    rl.add_history_entry(input.clone());
    Ok(input)
}

fn get_prompt(state: &mut State) -> Result<String, ReadError> {
    if let Some(p) = state.variables.get("PROMPT") {
        println!("READING prompt {}", p);
        let expanded_prompt = match parse_initial_cmd(&p.clone(), state) {
            Ok(v) => v,
            Err(e) => return Ok(format!("vrsh: failed to parse prompt: {}", e).to_string())
        };
        return Ok(expanded_prompt)
    }

    let curr_dir = current_dir()?;
    let wd = match curr_dir.to_str() {
        Some(dir) => dir,
        None => return Err(ReadError::NoWorkingDir),
    };

    let prompt = wd.replace(state.home.as_str(), HOME);

    return Ok(format!("{} > ", prompt))
}