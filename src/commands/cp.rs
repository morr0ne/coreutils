use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn cp(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "cp",
        "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
