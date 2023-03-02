use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn env(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "env",
        "Set each NAME to VALUE in the environment and run COMMAND",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
