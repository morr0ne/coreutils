use std::process::ExitCode;

use coreutils::{commands, Error, Result};

fn main() -> Result {
    let mut args = std::env::args();

    if let Some(arg) = args.nth(1) {
        match arg.as_str() {
            #[cfg(feature = "arch")]
            "arch" => commands::arch(args, true),
            #[cfg(feature = "b2sum")]
            "b2sum" => commands::b2sum(args, true),
            #[cfg(feature = "whoami")]
            "whoami" => commands::whoami(args, true),
            #[cfg(feature = "tty")]
            "tty" => commands::tty(args, true),
            #[cfg(feature = "false")]
            "false" => Ok(ExitCode::FAILURE), // TODO: handle "--version" and "--help".
            #[cfg(feature = "true")]
            "true" => commands::r#true(args, true),
            #[cfg(feature = "yes")]
            "yes" => commands::yes(args, true),
            command => Err(Error::UnknownCommand(command.to_string())),
        }
    } else {
        Err(Error::NoCommand)
    }
}

// call_commands!{}
