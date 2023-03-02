use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mkdir(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "mkdir",
        "Create the DIRECTORY(ies), if they do not already exist",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
