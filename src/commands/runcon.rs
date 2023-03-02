use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn runcon(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "runcon",
        "Run a program in a different SELinux security context",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
