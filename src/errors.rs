use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    PromptNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PromptNotFound => write!(f, "Prompt not found"),
        }
    }
}

impl StdError for Error {}
