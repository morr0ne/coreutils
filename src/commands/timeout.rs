use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn timeout(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "timeout",
        "Start COMMAND, and kill it if still running after DURATION",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
