use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn rm(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "rm",
        "Remove (unlink) the FILE(s)",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
