use crate::shell::types::{Arg, Cmd, CmdPart, Redirect, InitialCmd, InitialCmdPart};
use crate::shell::rl_helper::RLHelper;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::{current_dir, var_os};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error;

use crate::grammar::CommandParser;
use crate::expansions::InitialCmdParser;
use crate::shell::handle_command::{CommandError, handle_sub_command};

pub enum ParseError {
    IO(std::io::Error),
    NoWorkingDir,
    NoHomeVar,
    Internal,
    RLError(ReadlineError),
    RLIgnore,
    LALRPopErr(String),
    EvaluationError(CommandError)
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

    let initial_cmd: InitialCmd = match InitialCmdParser::new().parse(&s) {
        Ok(val) => val,
        Err(e) => return Err(ParseError::LALRPopErr(e.to_string())),
    };

    let expanded = expand_initial_cmd(initial_cmd)?;

    let command: Cmd = match CommandParser::new().parse(&expanded) {
        Ok(val) => val,
        Err(e) => return Err(ParseError::LALRPopErr(e.to_string())),
    };

    rl.add_history_entry(s);
    return replacements(command);
}

fn expand_initial_cmd(cmd: InitialCmd) -> Result<String, ParseError> {
    let mut text = cmd.text;
    for part in cmd.parts.into_iter() {
        match part {
            InitialCmdPart::String(val) => {
                text = text + val.as_str();
            }
            InitialCmdPart::Calculation(cmd) => {
                let inner = expand_initial_cmd(cmd)?;
                text += evaluate_cmd(inner)?.as_str();
            }
        }
    }

    Ok(text)
}

fn evaluate_cmd(cmd: String) -> Result<String, ParseError> {
    let parsed: Cmd = match CommandParser::new().parse(&cmd) {
        Ok(val) => replacements(val)?,
        Err(e) => return Err(ParseError::LALRPopErr(e.to_string())),
    };

    Ok(handle_sub_command(parsed)?)
}

fn replacements(cmd: Cmd) -> Result<Cmd, ParseError> {
    let home_dir = get_home_dir()?;
    return Ok(Cmd {
        parts: match cmd
            .parts
            .into_iter()
            .map(|v| Ok(CmdPart {
                cmd: v.cmd,
                args: match v
                    .args
                    .into_iter()
                    .map(|a| match a {
                        Arg::Word(s) => Ok(Arg::Word(s.replace(HOME, home_dir.as_str()))),
                        Arg::String(_) => Ok(a),
                    })
                    .collect::<Result<Vec<Arg>, ParseError>>() {
                    Ok(args) => args,
                    Err(e) => return Err(e),
                },
                redirects: v
                    .redirects
                    .into_iter()
                    .map(|redirect| match redirect {
                        Redirect::In(file) => Redirect::In(file.replace(HOME, home_dir.as_str())),
                        Redirect::Out(file) => Redirect::Out(file.replace(HOME, home_dir.as_str())),
                    })
                    .collect(),
            }))
            .collect::<Result<Vec<CmdPart>, ParseError>>() {
            Ok(l) => l,
            Err(e) => return Err(e)
        }
    });
}

fn get_prompt() -> Result<String, ParseError> {
    let curr_dir = current_dir()?;
    let wd = match curr_dir.to_str() {
        Some(dir) => dir,
        None => return Err(ParseError::NoWorkingDir),
    };

    let home_dir = get_home_dir()?;
    let prompt = wd.replace(home_dir.as_str(), HOME);

    return Ok(prompt);
}

pub fn get_home_dir() -> Result<String, ParseError> {
    return match var_os("HOME") {
        Some(os_s) => {
            if os_s.is_empty() {
                return Err(ParseError::NoHomeVar);
            }
            match os_s.to_str() {
                None => Err(ParseError::Internal),
                Some(s) => Ok(s.to_string()),
            }
        }
        None => Err(ParseError::NoHomeVar),
    };
}
