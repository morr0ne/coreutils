use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn kill(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "kill",
        "Forcibly terminate a process",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
