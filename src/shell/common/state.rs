use std::collections::HashMap;
use std::{fmt};
use std::env::var_os;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum StateError {
    EnvVarNotSet(String),
    EnvVarEmpty(String),
    InvalidEnvVar(String),
}

impl Display for StateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StateError::EnvVarNotSet(var) => write!(f, "environment variable {} is not set", var),
            StateError::EnvVarEmpty(var) => write!(f, "environment variable {} is empty", var),
            StateError::InvalidEnvVar(var) => write!(f, "environment variable {} set to an invalid value", var),
        }
    }
}

const HOME: &str = "HOME";

#[derive(Debug, Clone)]
pub struct State {
    pub aliases: HashMap<String, String>,
    pub home: String
}

pub fn new_state() -> Result<State, StateError> {
    let home_dir = get_home_dir()?;

    Ok(State {
        aliases: HashMap::new(),
        home: home_dir,
    })
}

pub fn get_home_dir() -> Result<String, StateError> {
    return match var_os(HOME) {
        Some(os_s) => {
            if os_s.is_empty() {
                return Err(StateError::EnvVarEmpty(String::from(HOME)))
            }
            match os_s.to_str() {
                None => Err(StateError::InvalidEnvVar(String::from(HOME))),
                Some(s) => Ok(s.to_string()),
            }
        }
        None => Err(StateError::EnvVarNotSet(String::from(HOME)))
    };
}
