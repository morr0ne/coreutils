use std::{fmt::Debug, io::Error as IoError, process::ExitCode};

use lexopt::Error as ParseError;

pub type Result<T = ExitCode, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No command specified")]
    NoCommand,
    #[error("{0} is not a valid command")]
    UnknownCommand(String),
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error(transparent)]
    ParseError(#[from] ParseError),
}
