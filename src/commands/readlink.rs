use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn readlink(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "readlink",
        "Print value of a symbolic link or canonical file name",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
