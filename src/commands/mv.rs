use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mv(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "mv",
        "Rename SOURCE to DEST, or move SOURCE(s) to DIRECTORY",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
