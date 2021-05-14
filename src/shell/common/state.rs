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
const USER: &str = "USER";

#[derive(Debug, Clone)]
pub struct State {
    pub aliases: HashMap<String, String>,
    pub username: String,
    pub home: String,
    pub variables: HashMap<String, String>
}

pub fn new_state() -> Result<State, StateError> {
    let home_dir = get_env_variable(HOME)?;
    let username = get_env_variable(USER)?;

    Ok(State {
        aliases: HashMap::new(),
        username,
        home: home_dir,
        variables: HashMap::new(),

    })
}

fn get_env_variable(var: &str) -> Result<String, StateError> {
    return match var_os(var) {
        Some(os_s) => {
            if os_s.is_empty() {
                return Err(StateError::EnvVarEmpty(String::from(var)))
            }

            match os_s.to_str() {
                None => Err(StateError::InvalidEnvVar(String::from(var))),
                Some(s) => Ok(s.to_string())
            }
        }
        None => Err(StateError::EnvVarNotSet(String::from(var)))
    }
}
