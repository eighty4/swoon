use std::{fmt, result};

use crate::api::task;

pub enum Name {
    Bake,
    Init,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command_name = match self {
            Name::Bake => "bake",
            Name::Init => "init",
        };
        write!(f, "{}", command_name)?;
        Ok(())
    }
}

pub type Result = result::Result<(), Error>;

pub const SUCCESS: Result = Result::Ok(());

pub struct Error {
    pub cause: task::Error,
    pub alt_commands: Vec<Name>,
}

impl Error {
    pub fn result(error_str: &str) -> Result {
        Result::Err(Error { cause: task::Error::new(error_str), alt_commands: vec!() })
    }

    pub fn with_command_suggestions(error_str: &str, alt_commands: Vec<Name>) -> Result {
        Result::Err(Error { cause: task::Error::new(error_str), alt_commands })
    }
}

impl From<task::Error> for Error {
    fn from(cause: task::Error) -> Self {
        Error { cause, alt_commands: vec!() }
    }
}
