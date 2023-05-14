use std::{fmt::Debug, io::Error as IoError, process::ExitCode, str::Utf8Error};

use lexopt::Error as ParseError;

pub type Result<T = ExitCode, E = Error> = std::result::Result<T, E>;

pub enum Error {
    NoCommand,
    UnknownCommand(String),
    IoError(IoError),
    ParseError(ParseError),
    Utf8Error(Utf8Error),
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCommand => write!(formatter, "No command specified"),
            Self::UnknownCommand(command) => write!(formatter, "{command} is not a valid command"),
            Self::IoError(error) => error.fmt(formatter),
            Self::ParseError(error) => error.fmt(formatter),
            Self::Utf8Error(error) => error.fmt(formatter),
        }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Error(error)
    }
}
