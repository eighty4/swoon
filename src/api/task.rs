use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub const SUCCESS: Result<()> = Result::Ok(());

#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error { msg: String::from(msg) }
    }

    pub fn result<T>(msg: &str) -> Result<T> {
        Result::Err(Error::new(msg))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}

impl From<std::io::Error> for Error {
    fn from(f: std::io::Error) -> Self {
        Error { msg: f.to_string() }
    }
}
