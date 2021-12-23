use std::error::Error;
use std::fmt;

pub mod context;
pub mod config;
pub mod template;

pub fn swoon_error_result<T>(msg: &str) -> Result<T, SwoonError> {
    return Result::Err(SwoonError { msg: String::from(msg) });
}

#[derive(Debug)]
pub struct SwoonError {
    msg: String,
}

impl fmt::Display for SwoonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;
        return fmt::Result::Ok(());
    }
}

impl From<std::io::Error> for SwoonError {
    fn from(f: std::io::Error) -> Self {
        SwoonError { msg: f.to_string() }
    }
}

impl From<liquid::Error> for SwoonError {
    fn from(f: liquid::Error) -> Self {
        SwoonError { msg: f.to_string() }
    }
}

impl Error for SwoonError {}
