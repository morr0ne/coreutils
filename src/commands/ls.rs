use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn ls(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "ls",
        "List information about the FILEs (the current directory by default)",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
