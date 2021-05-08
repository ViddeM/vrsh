use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error;

use crate::grammar::CommandParser;
use crate::expansions::InitialCmdOrCommentParser;
use crate::replacements::ReplacementCmdParser;
use crate::shell::handle_command::{CommandError, handle_sub_command};
use crate::shell::common::types::{Cmd, InitialCmd, InitialCmdPart, ReplacementsCmd, ReplacementPart, InitialCmdOrComment};
use crate::shell::common::state::{State};

pub enum ParseError {
    IO(std::io::Error),
    LALRPopErr(String, String),
    EvaluationError(CommandError),
    Comment,
    InputEmpty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IO(e) => write!(f, "io error: '{}'", e),
            ParseError::LALRPopErr(s, pass) => write!(f, "failed parsing: '{}' in pass {}", s, pass),
            ParseError::EvaluationError(cmd_err) => write!(f, "failed to evaluate command: {}", cmd_err),
            ParseError::Comment => write!(f, "comment encountered, ignore"),
            ParseError::InputEmpty => write!(f, "input empty, ignore"),
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: Error) -> Self {
        ParseError::IO(err)
    }
}

impl From<CommandError> for ParseError {
    fn from (cmd_err: CommandError) -> Self { ParseError::EvaluationError(cmd_err) }
}

pub const HOME: &str = "~";

pub fn parse_input(input: String, state: &mut State) -> Result<Cmd, ParseError> {
    if input.is_empty() {
        return Err(ParseError::InputEmpty);
    }

    let expanded = parse_initial_cmd(&input, state)?;

    let command = evaluate_cmd(expanded, state)?;
    return Ok(command);
}

pub fn parse_initial_cmd(input: &str, state: &mut State) -> Result<String, ParseError> {
    let initial_cmd: InitialCmd = match InitialCmdOrCommentParser::new().parse(&input) {
        Ok(val) => match val {
            InitialCmdOrComment::InitialCmd(v) => v,
            InitialCmdOrComment::Comment => {
                return Err(ParseError::Comment)
            }
        },
        Err(e) => return Err(ParseError::LALRPopErr(e.to_string(), String::from("initial"))),
    };

    Ok(expand_initial_cmd(initial_cmd, state)?)
}

fn expand_initial_cmd(cmd: InitialCmd, state: &mut State) -> Result<String, ParseError> {
    let mut text = "".to_string();
    for part in cmd.parts.into_iter() {
        match part {
            InitialCmdPart::String(val) => {
                text = text + val.as_str();
            }
            InitialCmdPart::Calculation(cmd) => {
                let inner = expand_initial_cmd(cmd, state)?;
                let new_cmd = evaluate_cmd(inner, state)?;
                text += handle_sub_command(new_cmd, state)?.as_str();
            },
            InitialCmdPart::SingleQuotedString(str) => {
                text = text + str.as_str();
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
            ReplacementPart::String(s) => s.to_string(),
            ReplacementPart::Word(w) => perform_replacement(w.to_string(), state),
            ReplacementPart::Variable(var) => read_var(var.to_string(), state).to_string()
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

fn read_var(var: String, state: &State) -> &str {
    match state.variables.get(&var) {
        Some(val) => val,
        None => ""
    }
}
