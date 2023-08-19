use std::process::ExitCode;

use coreutils::{Error, Result};

fn main() -> Result {
    let mut args = std::env::args();

    if let Some(arg) = args.nth(1) {
        match arg.as_str() {
            "arch" => coreutils::commands::arch(args),
            "b2sum" => coreutils::commands::b2sum(args),
            "cksum" => coreutils::commands::cksum(args),
            "tty" => coreutils::commands::tty(args),
            "whoami" => coreutils::commands::whoami(args),
            "yes" => coreutils::commands::yes(args),
            _ => Err(Error::UnknownCommand(arg)),
        }
    } else {
        return Ok(ExitCode::SUCCESS);
    }
}
