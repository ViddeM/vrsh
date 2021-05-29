use crate::shell::common::colors::{
    bg_color, bg_color_code, fg_color, fg_color_code, reset_color, Color, ColorError,
};
use crate::shell::common::state::State;
use crate::shell::parse_command::{parse_initial_cmd, ParseError, HOME};
use crate::shell::prompt::modules::vcs::git::git::{get_git_prompt, GitError};
use crate::shell::prompt::prompt_types::{Argument, PromptCmd, PromptCmdPart, PromptEscape};
use crate::shell::rl_helper::RLHelper;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::current_dir;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error;

use crate::prompt::PromptCmdParser;

pub enum ReadError {
    Ignore,
    RLError(ReadlineError),
    LalrpopError(String),
    ParseError(ParseError),
    NoWorkingDir,
    IO(std::io::Error),
    ColorError(ColorError),
    GitError(GitError),
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::IO(e) => write!(f, "io error: '{}'", e),
            ReadError::NoWorkingDir => write!(f, "failed to retrieve current working directory"),
            ReadError::RLError(e) => write!(f, "readline encountered an error: {}", e),
            ReadError::LalrpopError(s) => write!(f, "lalrpop error: {}", s),
            ReadError::ParseError(e) => write!(f, "parse error: {}", e),
            ReadError::ColorError(e) => write!(f, "color error: {}", e),
            ReadError::GitError(e) => write!(f, "git error: {}", e),
            ReadError::Ignore => write!(f, "ignored error"),
        }
    }
}

impl From<std::io::Error> for ReadError {
    fn from(err: Error) -> Self {
        ReadError::IO(err)
    }
}

impl From<ColorError> for ReadError {
    fn from(c: ColorError) -> Self {
        ReadError::ColorError(c)
    }
}

impl From<ParseError> for ReadError {
    fn from(e: ParseError) -> Self {
        ReadError::ParseError(e)
    }
}

impl From<GitError> for ReadError {
    fn from(e: GitError) -> Self {
        ReadError::GitError(e)
    }
}

pub fn read_input(rl: &mut Editor<RLHelper>, state: &mut State) -> Result<String, ReadError> {
    let prompt = match get_prompt(state) {
        Ok(v) => v,
        Err(e) => {
            println!("vrsh: failed to parse prompt: {}", e);
            "> ".to_string()
        }
    };
    let input = match rl.readline(prompt.as_str()) {
        Ok(val) => val,
        Err(e) => {
            return match e {
                ReadlineError::Interrupted => Err(ReadError::Ignore),
                ReadlineError::Eof => Err(ReadError::Ignore),
                _ => Err(ReadError::RLError(e)),
            }
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
        return Ok(expanded_prompt);
    }

    let curr_dir = current_dir()?;
    let wd = match curr_dir.to_str() {
        Some(dir) => dir,
        None => return Err(ReadError::NoWorkingDir),
    };

    let prompt = wd.replace(state.home.as_str(), HOME);

    return Ok(format!("{} > ", prompt));
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
    Ok(match e {
        PromptEscape::EscapeChar => String::from("%"),
        PromptEscape::Username => state.username.clone(),
        PromptEscape::Cwd => {
            let curr_dir = current_dir()?;
            match curr_dir.to_str() {
                Some(dir) => dir.to_string(),
                None => return Err(ReadError::NoWorkingDir),
            }
        }
        PromptEscape::CwdHome => {
            let curr_dir = current_dir()?;
            let cwd = match curr_dir.to_str() {
                Some(dir) => dir,
                None => return Err(ReadError::NoWorkingDir),
            };
            cwd.replace(state.home.as_str(), &format!("{}", HOME)) // 🏠
        }
        PromptEscape::FGColorStart(color_arg) => match color_arg {
            Argument::Number(n) => fg_color_code(n),
            Argument::Word(w) => fg_color(Color::from_string(&w)?),
        },
        PromptEscape::FGColorEnd => reset_color(),
        PromptEscape::BGColorStart(color_arg) => match color_arg {
            Argument::Number(n) => bg_color_code(n),
            Argument::Word(w) => bg_color(Color::from_string(&w)?),
        },
        PromptEscape::BGColorEnd => reset_color(),
        PromptEscape::Git => get_git_prompt()?,
    })
}
