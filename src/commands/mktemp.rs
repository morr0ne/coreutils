use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mktemp(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "mktemp",
        "Create a temporary file or directory, safely, and print its name",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
