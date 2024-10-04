use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    Io(std::io::Error),
    Fetcher(String),
    Timeout,
    Http(reqwest::Error),
    NotFound,
}


impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
      CustomError::Io(err)
    }
}


impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> Self {
        CustomError::Http(err)
    }
}


impl StdError for CustomError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CustomError::Io(err) => Some(err),
            CustomError::Fetcher(_) => None,
            CustomError::Timeout => None,
            CustomError::Http(err) => Some(err),
            CustomError::NotFound => None,
        }
    }
}


impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Io(err) => write!(f, "io error: {}", err),
            CustomError::Fetcher(msg) => write!(f, "fetcher error: {}", msg),
            CustomError::Timeout => write!(f, "timeout"),
            CustomError::Http(err) => write!(f, "http error: {}", err),
            CustomError::NotFound => write!(f, "not found"),
        }
    }
}