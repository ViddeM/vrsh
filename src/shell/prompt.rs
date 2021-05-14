use crate::shell::common::state::State;
use std::env::current_dir;
use crate::shell::rl_helper::RLHelper;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::shell::parse_command::{parse_initial_cmd, HOME, ParseError};
use std::io::Error;
use crate::prompt::PromptCmdParser;
use crate::shell::common::prompt_types::{PromptCmd, PromptCmdPart, PromptEscape};

pub enum ReadError {
    Ignore,
    RLError(ReadlineError),
    LalrpopError(String),
    ParseError(ParseError),
    NoWorkingDir,
    IO(std::io::Error),
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::IO(e) => write!(f, "io error: '{}'", e),
            ReadError::NoWorkingDir => write!(f, "failed to retrieve current working directory"),
            ReadError::RLError(e) => write!(f, "readline encountered an error: {}", e),
            ReadError::LalrpopError(s) => write!(f, "lalrpop error: {}", s),
            ReadError::ParseError(e) => write!(f, "parse error: {}", e),
            ReadError::Ignore => write!(f, "ignored error"),
        }
    }
}

impl From<std::io::Error> for ReadError {
    fn from(err: Error) -> Self {
        ReadError::IO(err)
    }
}

impl From<ParseError> for ReadError {
    fn from(e: ParseError) -> Self { ReadError::ParseError(e) }
}

pub fn read_input(rl: &mut Editor<RLHelper>, state: &mut State) -> Result<String, ReadError> {
    let prompt = match get_prompt(state) {
        Ok(v) => v,
        Err(e) => {
            println!("vrsh: failed to parse prompt: {}", e);
            "> ".to_string()
        },
    };
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
        let expanded = match parse_initial_cmd(&p.clone(), state) {
            Ok(v) => v,
            Err(ParseError::Comment | ParseError::InputEmpty) => "".to_string(),
            Err(e) => return Err(ReadError::from(e)),
        };

        let expanded_prompt = prompt_expand(&expanded, state)?;
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

fn prompt_expand(input: &str, state: &mut State) -> Result<String, ReadError> {
    let prompt_cmd: PromptCmd = match PromptCmdParser::new().parse(input) {
        Ok(v) => v,
        Err(e) => return Err(ReadError::LalrpopError(e.to_string())),
    };

    let mut str = String::from("");
    for part in prompt_cmd.into_iter() {
        match part {
            PromptCmdPart::Escaped(e) => {
                str += &handle_prompt_escape(e, state)?;
            }
            PromptCmdPart::Cmd(s) => {
                str += &s;
            }
        }
    }

    Ok(str)
}

fn handle_prompt_escape(e: PromptEscape, state: &mut State) -> Result<String, ReadError> {
    match e {
        PromptEscape::EscapeChar => Ok(String::from("%")),
        PromptEscape::Username => Ok(state.username.clone()),
        PromptEscape::Cwd => {
            let curr_dir = current_dir()?;
            match curr_dir.to_str() {
                Some(dir) => Ok(dir.to_string()),
                None => Err(ReadError::NoWorkingDir),
            }
        },
        PromptEscape::CwdHome => {
            let curr_dir = current_dir()?;
            let cwd = match curr_dir.to_str() {
                Some(dir) => Ok(dir),
                None => Err(ReadError::NoWorkingDir),
            }?;
            Ok(cwd.replace(state.home.as_str(), HOME))
        },
    }
}