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

pub enum PromptError {
    Ignore,
    RLError(ReadlineError),
    LalrpopError(String),
    ParseError(ParseError),
    NoWorkingDir,
    IO(std::io::Error),
    ColorError(ColorError),
    GitError(GitError),
}

impl Display for PromptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PromptError::IO(e) => write!(f, "io error: '{}'", e),
            PromptError::NoWorkingDir => write!(f, "failed to retrieve current working directory"),
            PromptError::RLError(e) => write!(f, "readline encountered an error: {}", e),
            PromptError::LalrpopError(s) => write!(f, "lalrpop error: {}", s),
            PromptError::ParseError(e) => write!(f, "parse error: {}", e),
            PromptError::ColorError(e) => write!(f, "color error: {}", e),
            PromptError::GitError(e) => write!(f, "git error: {}", e),
            PromptError::Ignore => write!(f, "ignored error"),
        }
    }
}

impl From<std::io::Error> for PromptError {
    fn from(err: Error) -> Self {
        PromptError::IO(err)
    }
}

impl From<ColorError> for PromptError {
    fn from(c: ColorError) -> Self {
        PromptError::ColorError(c)
    }
}

impl From<ParseError> for PromptError {
    fn from(e: ParseError) -> Self {
        PromptError::ParseError(e)
    }
}

impl From<GitError> for PromptError {
    fn from(e: GitError) -> Self {
        PromptError::GitError(e)
    }
}

pub fn read_input(rl: &mut Editor<RLHelper>, state: &mut State) -> Result<String, PromptError> {
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
                ReadlineError::Interrupted => Err(PromptError::Ignore),
                ReadlineError::Eof => Err(PromptError::Ignore),
                _ => Err(PromptError::RLError(e)),
            }
        }
    };
    rl.add_history_entry(input.clone());
    Ok(input)
}

fn get_prompt(state: &mut State) -> Result<String, PromptError> {
    if let Some(p) = state.variables.get("PROMPT") {
        let expanded = match parse_initial_cmd(&p.clone(), state) {
            Ok(v) => v,
            Err(ParseError::Comment | ParseError::InputEmpty) => "".to_string(),
            Err(e) => return Err(PromptError::from(e)),
        };

        let expanded_prompt = prompt_expand(&expanded, state)?;
        return Ok(expanded_prompt);
    }

    let curr_dir = current_dir()?;
    let wd = match curr_dir.to_str() {
        Some(dir) => dir,
        None => return Err(PromptError::NoWorkingDir),
    };

    let prompt = wd.replace(state.home.as_str(), HOME);

    return Ok(format!("{} > ", prompt));
}

fn prompt_expand(input: &str, state: &mut State) -> Result<String, PromptError> {
    let prompt_cmd: PromptCmd = match PromptCmdParser::new().parse(input) {
        Ok(v) => v,
        Err(e) => return Err(PromptError::LalrpopError(e.to_string())),
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

fn handle_prompt_escape(e: PromptEscape, state: &mut State) -> Result<String, PromptError> {
    Ok(match e {
        PromptEscape::EscapeChar => String::from("%"),
        PromptEscape::Username => state.username.clone(),
        PromptEscape::Cwd => {
            let curr_dir = current_dir()?;
            match curr_dir.to_str() {
                Some(dir) => dir.to_string(),
                None => return Err(PromptError::NoWorkingDir),
            }
        }
        PromptEscape::CwdHome => {
            let curr_dir = current_dir()?;
            let cwd = match curr_dir.to_str() {
                Some(dir) => dir,
                None => return Err(PromptError::NoWorkingDir),
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
