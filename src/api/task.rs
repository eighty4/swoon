use std::{fmt, process};
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub const SUCCESS: Result<()> = Ok(());

pub fn fatal<S: AsRef<str>>(msg: S) -> ! {
    Error::new(msg).exit()
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Self { msg: String::from(msg.as_ref()) }
    }

    pub fn result<T, S: AsRef<str>>(msg: S) -> Result<T> {
        Result::Err(Self::new(msg.as_ref()))
    }

    pub fn exit(&self) -> ! {
        println!("{}", self.to_string());
        process::exit(1);
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
        Self::new(f.to_string())
    }
}
