use std::convert::From;
use std::error::Error as StdError;
use std::fmt as std_fmt;
use std::fmt::Display;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    General,
    ValidityFail,
    InvalidId,
    IllegalPosition,
    EntryExist,
    ArgError,
    Io,
    Format,
    SerdeJson
}

#[derive(Debug, Clone)]
pub struct Error {
    /// Formatted error message
    pub message: String,
    /// The type of error
    pub kind: ErrorKind,
    /// Any additional information passed along, such as the argument name that caused the error
    pub info: Option<Vec<String>>,
}

impl Error {
    pub fn general(description: &str) -> Self {
        Error::with_description(description, ErrorKind::General)
    }
    pub fn validity_fail() -> Self {
        Error {
            message: format!("error: Validity check failed."),
            kind: ErrorKind::ValidityFail,
            info: None
        }
    }
    pub fn invalid_id() -> Self {
        Error {
            message: format!("error: Id not found."),
            kind: ErrorKind::InvalidId,
            info: None
        }
    }
    pub fn illegal_position(tar: usize, len: usize) -> Self {
        Error {
            message: format!("error: Illegal position: {} of {}.", tar, len),
            kind: ErrorKind::IllegalPosition,
            info: Some(vec![format!("{:?}", (tar, len))])
        }
    }
    pub fn entry_exist() -> Self {
        Error {
            message: format!("error: Entry already exists."),
            kind: ErrorKind::EntryExist,
            info: None
        }
    }
    pub fn arg_error(cmd: &str, arg: &str) -> Self {
        Error {
            message: format!("Command: {}, invalid arg: {}.", cmd, arg),
            kind: ErrorKind::ArgError,
            info: Some(vec![cmd.to_string(), arg.to_string()])
        }
    }
    pub fn with_description(description: &str, kind: ErrorKind) -> Self {
        Error {
            message: format!("{} {}", "error:", description),
            kind,
            info: None,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &*self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std_fmt::Formatter) -> std_fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::with_description(&*e.to_string(), ErrorKind::ArgError)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::with_description(&*e.to_string(), ErrorKind::Io)
    }
}

impl From<std_fmt::Error> for Error {
    fn from(e: std_fmt::Error) -> Self {
        Error::with_description(&*e.to_string(), ErrorKind::Format)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::with_description(&*e.to_string(), ErrorKind::SerdeJson)
    }
}
