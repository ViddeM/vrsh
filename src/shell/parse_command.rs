use crate::shell::rl_helper::RLHelper;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::{current_dir};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error;

use crate::grammar::CommandParser;
use crate::expansions::InitialCmdParser;
use crate::replacements::ReplacementCmdParser;
use crate::shell::handle_command::{CommandError, handle_sub_command};
use crate::shell::common::types::{Cmd, InitialCmd, InitialCmdPart, ReplacementsCmd, ReplacementPart};
use crate::shell::common::state::{State};

pub enum ParseError {
    IO(std::io::Error),
    NoWorkingDir,
    RLError(ReadlineError),
    RLIgnore,
    LALRPopErr(String, String),
    EvaluationError(CommandError)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IO(e) => write!(f, "io error: '{}'", e),
            ParseError::NoWorkingDir => write!(f, "failed to retrieve current working directory"),
            ParseError::RLError(e) => write!(f, "readline encountered an error: {}", e),
            ParseError::LALRPopErr(s, pass) => write!(f, "failed parsing: {} in pass {}", s, pass),
            ParseError::RLIgnore => write!(f, "ignored error"),
            ParseError::EvaluationError(cmd_err) => write!(f, "failed to evaluate command: {}", cmd_err)
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

impl From<CommandError> for ParseError {
    fn from (cmd_err: CommandError) -> Self { ParseError::EvaluationError(cmd_err) }
}

const HOME: &str = "~";

pub fn parse_input(rl: &mut Editor<RLHelper>, state: &mut State) -> Result<Cmd, ParseError> {
    let prompt_prefix = get_prompt(state)?;
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

    let initial_cmd: InitialCmd = match InitialCmdParser::new().parse(&s) {
        Ok(val) => val,
        Err(e) => return Err(ParseError::LALRPopErr(e.to_string(), String::from("initial"))),
    };
    let expanded = expand_initial_cmd(initial_cmd, state)?;

    let command = evaluate_cmd(expanded, state)?;

    rl.add_history_entry(s);
    return Ok(command);
}

fn expand_initial_cmd(cmd: InitialCmd, state: &mut State) -> Result<String, ParseError> {
    let mut text = cmd.text;
    for part in cmd.parts.into_iter() {
        match part {
            InitialCmdPart::String(val) => {
                text = text + val.as_str();
            }
            InitialCmdPart::Calculation(cmd) => {
                let inner = expand_initial_cmd(cmd, state)?;
                let new_cmd = evaluate_cmd(inner, state)?;
                text += handle_sub_command(new_cmd, state)?.as_str();
            }
        }
    }

    Ok(text)
}

fn evaluate_cmd(cmd: String, state: &mut State) -> Result<Cmd, ParseError> {
    let replaced = perform_replacements(cmd, state)?;

    match CommandParser::new().parse(&replaced) {
        Ok(val) => Ok(val),
        Err(e) => Err(ParseError::LALRPopErr(e.to_string(), String::from("command"))),
    }
}

fn perform_replacements(str: String, state: &State) -> Result<String, ParseError> {
    let replaced_cmd: ReplacementsCmd = match ReplacementCmdParser::new().parse(&str) {
        Ok(val) => val,
        Err(e) => return Err(ParseError::LALRPopErr(e.to_string(), String::from("replacements"))),
    };
    Ok(handle_replaced_cmd(replaced_cmd, state))
}

fn handle_replaced_cmd(replacement_cmd: ReplacementsCmd, state: &State) -> String {
    let mut replaced_str = String::new();
    for part in replacement_cmd.parts.iter() {
        let val = match part {
            ReplacementPart::String(s) => s.to_owned(),
            ReplacementPart::Word(w) => perform_replacement(w.to_string(), state)
        };
        replaced_str = replaced_str + &val;
    }
    replaced_str
}

fn perform_replacement(str: String, state: &State) -> String {
    let mut replaced = str.clone();
    for (key, val) in state.aliases.iter() {
        replaced = replaced.replace(key, val);
    }
    replaced.replace(HOME, state.home.as_str())
}

fn get_prompt(state: &State) -> Result<String, ParseError> {
    let curr_dir = current_dir()?;
    let wd = match curr_dir.to_str() {
        Some(dir) => dir,
        None => return Err(ParseError::NoWorkingDir),
    };

    let prompt = wd.replace(state.home.as_str(), HOME);

    return Ok(prompt);
}