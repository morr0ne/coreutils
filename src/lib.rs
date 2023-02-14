use std::{fmt::Debug, io::Error as IoError, process::ExitCode};

pub mod commands;
mod util;

pub type Result<T = ExitCode, E = Error> = std::result::Result<T, E>;

pub enum Error {
    NoCommand,
    UnknownCommand(String),
    IoError(IoError),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCommand => write!(f, "No command specified"),
            Self::UnknownCommand(command) => write!(f, "{command} is not a valid command"),
            Self::IoError(error) => error.fmt(f),
        }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}
