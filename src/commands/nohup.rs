use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn nohup(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "nohup",
        "Run COMMAND, ignoring hangup signals",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
