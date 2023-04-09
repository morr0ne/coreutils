use std::process::ExitCode;

use coreutils::{Error, Result};

fn main() -> Result {
    let mut args = std::env::args();

    if let Some(arg) = args.nth(1) {
        match arg.as_str() {
            #[cfg(feature = "arch")]
            "arch" => coreutils::commands::arch(args),
            #[cfg(feature = "b2sum")]
            "b2sum" => coreutils::commands::b2sum(args),
            #[cfg(feature = "cksum")]
            "cksum" => coreutils::commands::cksum(args),
            #[cfg(feature = "tty")]
            "tty" => coreutils::commands::tty(args),
            #[cfg(feature = "whoami")]
            "whoami" => coreutils::commands::whoami(args),
            #[cfg(feature = "yes")]
            "yes" => coreutils::commands::yes(args),
            _ => Err(Error::UnknownCommand(arg)),
        }
    } else {
        return Ok(ExitCode::SUCCESS);
    }
}
